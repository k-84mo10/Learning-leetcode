class Solution {
    func simplifyPath(_ path: String) -> String {
        let parts = path.split(separator: "/", omittingEmptySubsequences: true)
        var stack: [Substring] = []

        for part in parts {
            switch part {
                case ".":
                    continue
                case "..":
                    if !stack.isEmpty{
                        stack.removeLast()
                    }
                default:
                    stack.append(part)
            }
        }

        return "/" + stack.joined(separator: "/")
    }
}