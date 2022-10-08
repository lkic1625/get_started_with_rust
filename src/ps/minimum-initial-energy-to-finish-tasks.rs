use std::cmp;

impl Solution {
    pub fn minimum_effort(tasks: Vec<Vec<i32>>) -> i32 {
        let mut v = tasks;
        v.sort_by_key(|v| v[1] - v[0]);

        let mut ans = 0;
        for task in v {
            ans = cmp::max(task[1], ans + task[0]);
        }

        return ans;
    }
}
