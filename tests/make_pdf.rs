use awesome_docs::{
    self, content::ContentBuilder, document::DocumentBuilder, docx::Docx, pdf::Pdf,
};

#[test]
fn ordered_list_pdf() {
    let first_line = "First Paragraph";

    let second_line = "Another paragraph, this time a little bit longert to make sure this line will be divided into at least two lines";

    let doc_definition = ContentBuilder::new()
        .ordered_list()
        .add_child(first_line)
        .add_child(second_line)
        .build();

    let pdf_builder: Pdf = DocumentBuilder::new()
        .title("Test Pdf")
        .with_definition(doc_definition)
        .build();
}

#[test]
fn unordered_list_pdf() {
    let first_line = "First Paragraph";

    let second_line = "Another paragraph, this time a little bit longert to make sure this line will be divided into at least two lines";

    let doc_definition = ContentBuilder::new()
        .unordered_list()
        .add_child(first_line)
        .add_child(second_line)
        .build();

    let pdf_builder: Pdf = DocumentBuilder::new()
        .title("Test Pdf")
        .with_definition(doc_definition)
        .build();
}

fn single_line_pdf() {
    let first_line = "First Paragraph";

    let doc_definition = ContentBuilder::new().add_child(first_line).build();

    let pdf_builder: Pdf = DocumentBuilder::new()
        .title("Test Pdf")
        .with_definition(doc_definition)
        .build();
}

#[test]
fn ordered_list_docx() {
    let first_line = "First Paragraph";

    let second_line = "Another paragraph, this time a little bit longert to make sure this line will be divided into at least two lines";

    let doc_definition = ContentBuilder::new()
        .ordered_list()
        .add_child(first_line)
        .add_child(second_line)
        .build();

    let pdf_builder: Docx = DocumentBuilder::new()
        .title("Test Docx")
        .with_definition(doc_definition)
        .build();
}

/*
#[test]
fn basic_ppt() {
    let first_line = "First Paragraph";

    let second_line = "Another paragraph, this time a little bit longert to make sure this line will be divided into at least two lines";

    let doc_definition = ContentBuilder::new()
        .add_child(first_line)
        .add_child(second_line)
        .build();

    let pdf_builder: Pptx = DocumentBuilder::new()
        .title("Test Pdf")
        .with_definition(doc_definition)
        .build();
}
*/
