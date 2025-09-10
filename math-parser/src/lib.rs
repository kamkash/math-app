pub mod gen_parsers;
use log::info;



pub fn echo_parser(name: &str) -> String {
    info!("parser echo {}", name);
    format!("{}", name)
}


/// Extracts all LaTeX math blocks (inline and display) from a LaTeX document.
pub fn extract_math_blocks(doc: &str) -> Vec<String> {
    // Regex covers inline `$...$`, display `$$...$$`, `$begin:math:display$...$end:math:display$`, and environments like `equation`
    let re = fancy_regex::Regex::new(
        r"(?xs)
        (                           # capture group
            \$\$(?P<dollar>.+?)\$\$ |     # $$...$$
            \$(?P<inline>[^$]+?)\$ |      # $...$
            \\\[(?P<bracket>.+?)\\\] |    # \[...\]
            \\begin\{(?P<env>\w+)\}(?P<env_content>.+?)\\end\{\k<env>\} # \begin{env}...\end{env}
        )"
    ).unwrap();

    let mut results = Vec::new();
    for cap in re.captures_iter(doc) {
        let res = cap.expect("Failed to capture regex group");
        if let Some(m) = res.name("dollar") {
            results.push(m.as_str().to_string());
        } else if let Some(m) = res.name("inline") {
            results.push(m.as_str().to_string());
        } else if let Some(m) = res.name("bracket") {
            results.push(m.as_str().to_string());
        } else if let (Some(_), Some(content)) = (res.name("env"), res.name("env_content")) {
            results.push(content.as_str().to_string());
        }
    }
    results
}
