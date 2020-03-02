use cc;

fn main() {
    cc::Build::new()
        .file("src/doublemap.c")
        .compile("doublemap");
}
