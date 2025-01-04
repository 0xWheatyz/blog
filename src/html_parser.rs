use std::fs;

pub fn convert_wmd_to_html(path: String) -> String{
    convert_to_html(
    input_parse(
    input_read(
        path
    )))
}

fn input_read(path: String) -> String {
    match fs::read_to_string(path) {
        Ok(content) => content,
        Err(_e) => fs::read_to_string("/var/www/error.html").expect("Read string from error")
    }
}

// Parse string into object containing different styling types
struct Paragraph {
    content: String
}

struct Title {
    content: String
}

struct Image {
    image_path: String,
    alt_text: String
}

struct Subheading {
    content: String
}

enum Content {
    Paragraph(Paragraph),
    Title(Title),
    Image(Image),
    Subheading(Subheading)
}

fn input_parse(page: String) -> Vec<Content> {
    let lines: Vec<&str> = page.split("\n").collect();
    
    let mut page: Vec<Content> = Vec::new();

    for part in lines {
        let new_part: Vec<&str> = part.split(&['[', ']'][..]).collect();

        // Make sure its not an empty line
        if new_part.len() == 1 {
            continue
        }

        let line_contents: String = new_part[2].to_string();

        if new_part[1].contains("p") {
            let item: Paragraph = Paragraph {
                content: line_contents
            };

            page.push(Content::Paragraph(item));

        } else if new_part[1].contains("i") {
            // More complicated due to splitting alt text and img src
            let portions: Vec<&str> = line_contents.split("|").collect();

            let img_src: String = portions[0].to_string();
            let alt_txt: String = portions[1].to_string();

            let item: Image = Image {
                image_path: img_src,
                alt_text: alt_txt,
            };
            
            page.push(Content::Image(item));

        } else if new_part[1].contains("t") {
            let item: Title = Title {
                content: line_contents,
            };

            page.push(Content::Title(item));

        } else if new_part[1].contains("s") {
            let item: Subheading = Subheading {
                content: line_contents,
            };

            page.push(Content::Subheading(item))
        }
    }

    page
}

// Parse the page object and convert to html 
fn convert_to_html(elements: Vec<Content>) -> String {
    let mut full_page: Vec<String> = Vec::new();

    let header: String = fs::read_to_string("/var/www/stock_html/header.html").expect("failed to read from header");
    let footer: String = fs::read_to_string("/var/www/stock_html/footer.html").expect("failed to read from footer");

    // Append the header to the full page
    full_page.push(header);

    for element in elements {
        let clean_elem = match element {
            Content::Paragraph(paragraph) => format!("<p>{}</p>", paragraph.content),
            Content::Image(image) => format!("<img src='{}' alt='{}'>", image.image_path, image.alt_text),
            Content::Title(title) => format!("<h1>{}</h1>", title.content),
            Content::Subheading(heading) => format!("<h2>{}</h2>", heading.content),
        };
        // Combine all the clean_elem strings to the full_page string 
        full_page.push(clean_elem);
    }

    // Append the footer to the full page
    full_page.push(footer);

    full_page
        .join("\n")
        .to_string()
}

