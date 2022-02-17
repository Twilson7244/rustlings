// strings1.rs
// Make me compile without changing the function signature!
// Execute `rustlings hint strings1` for hints ;)

// I AM NOT DONE
@@ -0,0 +1,7 @@
root = true

[*.rs]
end_of_line = lf
insert_final_newfile = true
indent_style = space
indent_size = 4
fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    "blue"
}
