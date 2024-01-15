import * as test from "imparlance-md";
let returned = test.greet("Trevor");
console.log(returned);

let markdown = "# Header 1\n## Header 2\n- List 1\n- List 2";
let result = test.parse_markdown(markdown);
console.log(result);
