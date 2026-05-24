func repeatedSubstringPattern(s string) bool {
    doubled := s + s
    trimmed := doubled[1 : len(doubled) - 1]

    return strings.Contains(trimmed, s)
}