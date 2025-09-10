use log::info;
use math_parser::extract_math_blocks;

#[test_log::test]
fn test_extract_math_blocks() {
    let tex_doc = r#"
        \section{Introduction}
        This paper analyzes the quadratic $ax^2+bx+c=0$.
        
        \[
            x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}
        \]

        And we also show:
        \begin{equation}
            \int_0^\infty e^{-x^2}\,dx = \frac{\sqrt{\pi}}{2}
        \end{equation}
    "#;

    let math_blocks = extract_math_blocks(tex_doc);
    for (i, block) in math_blocks.iter().enumerate() {
        info!("Block {}: {}", i + 1, block);
    }
    assert!(math_blocks.len() == 3);
    assert!(math_blocks
        .iter()
        .any(|block| block.contains(&"ax^2+bx+c=0".to_string())));
    assert!(math_blocks
        .iter()
        .any(|block| block.contains(&r"x = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}".to_string())));
    assert!(math_blocks
        .iter()
        .any(|block| block
            .contains(&r"\int_0^\infty e^{-x^2}\,dx = \frac{\sqrt{\pi}}{2}".to_string())));
}
