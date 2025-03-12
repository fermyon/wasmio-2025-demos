export const sorting = {
    sort: (input) => {
        let n = input.length;
        for (let i = 0; i < n - 1; i++) {
            for (let j = 0; j < n - i - 1; j++) {
                if (input[j] > input[j + 1]) {
                    [input[j], input[j + 1]] = [input[j + 1], input[j]];
                }
            }
        }
        return input;
    }
}