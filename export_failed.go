package main

import (
    "database/sql"
    "flag"
    "fmt"
    "os"
    "path/filepath"
    "regexp"
    "strings"

    _ "github.com/go-sql-driver/mysql"
)

type evalRow struct {
    ID         int
    Model      string
    Lang       sql.NullString
    ProblemID  int
    Response   sql.NullString
    ContestID  int
    IndexName  sql.NullString
}

func main() {
    dbDSN := flag.String("db", "user:pass@tcp(127.0.0.1:3306)/dbname", "Database DSN")
    modelsCSV := flag.String("models", "grok-4,gemini-2.5-pro,gpt-5-mini,o3,gpt-5", "Comma-separated model filters")
    dryRun := flag.Bool("dry-run", false, "Only print what would be written")
    flag.Parse()

    // Normalize model filters
    var modelFilters []string
    for _, m := range strings.Split(*modelsCSV, ",") {
        m = strings.TrimSpace(m)
        if m != "" {
            modelFilters = append(modelFilters, m)
        }
    }
    if len(modelFilters) == 0 {
        fmt.Println("No model filters provided")
        os.Exit(1)
    }

    db, err := sql.Open("mysql", *dbDSN)
    if err != nil {
        panic(err)
    }
    defer db.Close()

    // Build WHERE clause for exact model matches
    var placeholders []string
    var args []interface{}
    for _, mf := range modelFilters {
        placeholders = append(placeholders, "?")
        args = append(args, mf)
    }
    whereModels := "e.model IN (" + strings.Join(placeholders, ",") + ")"

    query := `
        SELECT e.id, e.model, e.lang, e.problem_id, e.response, p.contest_id, p.index_name
        FROM evaluations e
        JOIN problems p ON e.problem_id = p.id
        WHERE e.success = FALSE AND ` + whereModels + `
        ORDER BY e.id ASC`

    rows, err := db.Query(query, args...)
    if err != nil {
        panic(err)
    }
    defer rows.Close()

    count := 0
    for rows.Next() {
        var r evalRow
        if err := rows.Scan(&r.ID, &r.Model, &r.Lang, &r.ProblemID, &r.Response, &r.ContestID, &r.IndexName); err != nil {
            panic(err)
        }

        // Determine language
        lang := ""
        if r.Lang.Valid {
            lang = strings.ToLower(strings.TrimSpace(r.Lang.String))
        }
        switch lang {
        case "py":
            lang = "python"
        case "rs":
            lang = "rust"
        case "cpp":
            lang = "c++"
        }

        // Extract response string
        resp := ""
        if r.Response.Valid {
            resp = r.Response.String
        }
        // If language is unknown, try to guess from response
        if lang == "" {
            guessedLang, _ := guessLangFromResponse(resp)
            lang = guessedLang
        }

        ext := extFromLang(lang)
        if ext == "" {
            fmt.Printf("Skipping eval %d: unknown or unsupported language (lang='%s')\n", r.ID, lang)
            continue
        }

        code := extractCode(resp, lang)
        if strings.TrimSpace(code) == "" {
            fmt.Printf("Skipping eval %d: empty code after extraction\n", r.ID)
            continue
        }

        // Model folder (sanitized)
        dirName := sanitizeDirName(r.Model)
        if !*dryRun {
            if err := os.MkdirAll(dirName, 0o755); err != nil {
                panic(err)
            }
        }

        if !r.IndexName.Valid {
            fmt.Printf("Skipping eval %d: missing problem index\n", r.ID)
            continue
        }
        baseName := fmt.Sprintf("%d%s%s", r.ContestID, r.IndexName.String, ext)
        outPath := filepath.Join(dirName, baseName)

        // Avoid overwriting existing files; if exists, suffix with eval ID
        if !*dryRun {
            if _, err := os.Stat(outPath); err == nil {
                outPath = filepath.Join(dirName, fmt.Sprintf("%d%s_id%d%s", r.ContestID, r.IndexName.String, r.ID, ext))
            }
            if err := os.WriteFile(outPath, []byte(code), 0o644); err != nil {
                panic(err)
            }
        }
        fmt.Printf("Saved: %s (model=%s, lang=%s, eval_id=%d)\n", outPath, r.Model, lang, r.ID)
        count++
    }

    if err := rows.Err(); err != nil {
        panic(err)
    }

    fmt.Printf("Done. Exported %d failed solutions.\n", count)
}

func extFromLang(lang string) string {
    switch strings.ToLower(lang) {
    case "go":
        return ".go"
    case "python", "py":
        return ".py"
    case "rust", "rs":
        return ".rs"
    case "java":
        return ".java"
    case "c":
        return ".c"
    case "c++", "cpp":
        return ".cpp"
    default:
        return ""
    }
}

// extractCode attempts to pull the code block for a given language, falling back to the first fenced block
func extractCode(response, language string) string {
    // Try ```<language> ... ```
    re := regexp.MustCompile(fmt.Sprintf(`(?s)\x60\x60\x60%s\s*(.*?)\x60\x60\x60`, regexp.QuoteMeta(language)))
    if m := re.FindStringSubmatch(response); len(m) > 1 {
        return strings.TrimSpace(m[1])
    }
    // Try unlabeled fenced block
    re = regexp.MustCompile(`(?s)\x60\x60\x60\s*(.*?)\x60\x60\x60`)
    if m := re.FindStringSubmatch(response); len(m) > 1 {
        return strings.TrimSpace(m[1])
    }
    return strings.TrimSpace(response)
}

// guessLangFromResponse attempts to infer a language and extension from the fenced code label
// or via simple heuristics in the text.
func guessLangFromResponse(resp string) (string, string) {
    // Look for ```<lang> fences
    re := regexp.MustCompile(`(?i)\x60\x60\x60\s*([a-z0-9+#._-]+)\s*\n`)
    if m := re.FindStringSubmatch(resp); len(m) > 1 {
        lang := normalizeLangToken(m[1])
        if ext := extFromLang(lang); ext != "" {
            return lang, ext
        }
    }
    s := resp
    lower := strings.ToLower(s)
    // Heuristics
    switch {
    case strings.Contains(lower, "package main") || strings.Contains(lower, "func main("):
        return "go", ".go"
    case strings.Contains(lower, "#include <iostream>") || strings.Contains(lower, "using namespace std"):
        return "c++", ".cpp"
    case strings.Contains(lower, "#include"):
        return "c", ".c"
    case strings.Contains(lower, "public static void main(") || strings.Contains(lower, "class Main"):
        return "java", ".java"
    case strings.Contains(lower, "fn main()") || strings.Contains(lower, "use std::"):
        return "rust", ".rs"
    case strings.Contains(lower, "#!/usr/bin/env python") || strings.Contains(lower, "def main(") || strings.Contains(lower, "sys.stdin") || strings.Contains(lower, "input("):
        return "python", ".py"
    }
    return "", ""
}

func normalizeLangToken(tok string) string {
    t := strings.ToLower(strings.TrimSpace(tok))
    switch t {
    case "py", "python3":
        return "python"
    case "golang":
        return "go"
    case "rs":
        return "rust"
    case "cpp", "cxx", "cc", "cplusplus", "c++17", "c++20":
        return "c++"
    default:
        return t
    }
}

func sanitizeDirName(name string) string {
    // Replace path separators and spaces with underscores; trim
    name = strings.TrimSpace(name)
    name = strings.ReplaceAll(name, string(os.PathSeparator), "_")
    // Also replace characters that might be awkward in dir names
    repl := []string{"/", "\\", " ", ":", "*", "?", "\"", "<", ">", "|"}
    for _, r := range repl {
        name = strings.ReplaceAll(name, r, "_")
    }
    // Collapse multiple underscores
    for strings.Contains(name, "__") {
        name = strings.ReplaceAll(name, "__", "_")
    }
    if name == "" {
        name = "unknown_model"
    }
    return name
}
