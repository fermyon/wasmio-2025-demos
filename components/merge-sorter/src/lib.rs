use bindings::exports::wasmio::demo::sorting::Guest;

#[allow(warnings)]
mod bindings;

struct Component;
impl Component {
    fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(left.len() + right.len());
        let (mut i, mut j) = (0, 0);

        while i < left.len() && j < right.len() {
            if left[i] < right[j] {
                result.push(left[i]);
                i += 1;
            } else {
                result.push(right[j]);
                j += 1;
            }
        }

        result.extend_from_slice(&left[i..]);
        result.extend_from_slice(&right[j..]);

        result
    }
}
impl Guest for Component {
    fn sort(input: Vec<i32>) -> Vec<i32> {
        if input.len() <= 1 {
            return input;
        }

        let mid = input.len() / 2;
        let left = Component::sort(input[..mid].to_vec());
        let right = Component::sort(input[mid..].to_vec());

        Component::merge(left, right)
    }
}

bindings::export!(Component with_types_in bindings);
