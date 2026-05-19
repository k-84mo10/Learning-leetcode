func getCommon(nums1 []int, nums2 []int) int {
    len1 := len(nums1)
    len2 := len(nums2)

    idx1 := 0
    idx2 := 0

    for idx1 < len1 && idx2 < len2 {
        n1 := nums1[idx1]
        n2 := nums2[idx2]

        if n1 == n2 {
            return n1
        } else if n1 < n2 {
            idx1 += 1
        } else {
            idx2 += 1
        }
    }

    return -1
}