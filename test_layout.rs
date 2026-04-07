// Test to predict TUI layout and clock positioning
// Terminal size from alacritty.toml: 35 columns × 23 lines

fn main() {
    let total_lines = 23;
    let total_cols = 35;

    println!("=== TUI Layout Test ===");
    println!("Terminal: {}×{}", total_cols, total_lines);
    println!();

    // Account for outer border (1 line top, 1 line bottom)
    let inner_lines = total_lines - 2;
    println!("After borders: {} inner lines", inner_lines);
    println!();

    // Main layout chunks (with Min constraints, space is distributed)
    let tabs = 1;
    let spacer1 = 1;
    let message = 1;
    let min_content = 10;
    let min_clock = 5;

    // Calculate actual distribution
    let remaining = inner_lines - tabs - spacer1 - message;
    let content = if remaining <= min_content + min_clock {
        min_content
    } else {
        // Distribute remaining space evenly between content and clock
        remaining / 2
    };
    let clock_area = remaining - content;

    println!("Layout chunks:");
    println!("  Tabs:        {} line(s)  (lines 1-{})", tabs, tabs);
    println!("  Spacer:      {} line(s)  (lines {}-{})", spacer1, tabs + 1, tabs + spacer1);
    println!("  Content:     {} line(s)  (lines {}-{})", content, tabs + spacer1 + 1, tabs + spacer1 + content);
    println!("  Clock area:  {} line(s)  (lines {}-{})", clock_area, tabs + spacer1 + content + 1, tabs + spacer1 + content + clock_area);
    println!("  Message:     {} line(s)  (lines {}-{})", message, inner_lines, inner_lines);
    println!();

    // Clock area subdivision (7 lines total)
    let clock_start_line = tabs + spacer1 + content + 1;
    let clock_lines = 5;
    let remaining = clock_area - clock_lines;
    let top_padding = remaining / 2;
    let bottom_padding = remaining - top_padding;

    println!("Clock area subdivision ({} lines):", clock_area);
    println!("  Top padding:    {} line(s)", top_padding);
    println!("  Clock (ASCII):  {} line(s)  (lines {}-{})",
             clock_lines,
             clock_start_line + top_padding,
             clock_start_line + top_padding + clock_lines - 1);
    println!("  Bottom padding: {} line(s)", bottom_padding);
    println!();

    // Visual representation
    println!("=== Visual Layout ===");
    for i in 1..=total_lines {
        if i == 1 || i == total_lines {
            println!("{:2} │ ┌─── Border ───┐", i);
        } else {
            let inner_i = i - 1;
            let label = if inner_i >= 1 && inner_i < 1 + tabs {
                "Tabs"
            } else if inner_i >= 1 + tabs && inner_i < 1 + tabs + spacer1 {
                "Spacer"
            } else if inner_i >= 1 + tabs + spacer1 && inner_i < 1 + tabs + spacer1 + content {
                "Content"
            } else if inner_i >= clock_start_line && inner_i < clock_start_line + top_padding {
                "Clock [padding]"
            } else if inner_i >= clock_start_line + top_padding && inner_i < clock_start_line + top_padding + clock_lines {
                "Clock [ASCII]"
            } else if inner_i >= clock_start_line + top_padding + clock_lines && inner_i < clock_start_line + clock_area {
                "Clock [padding]"
            } else if inner_i == inner_lines {
                "Message"
            } else {
                "???"
            };
            println!("{:2} │ {}", i, label);
        }
    }
    println!();

    // Check if clock is centered
    println!("=== Verification ===");
    if top_padding == bottom_padding {
        println!("✓ Clock is vertically centered ({} lines padding on each side)", top_padding);
    } else {
        println!("✗ Clock is NOT centered (top: {}, bottom: {})", top_padding, bottom_padding);
    }
}
