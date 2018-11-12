use std::io;

use termesh::drawille::Canvas;
use termesh::stl::Stl;

#[test]
fn test_teapot() {
    let teapot_stl = include_bytes!("../data/teapot.stl");

    let stl = Stl::parse_binary(&mut io::Cursor::new(&teapot_stl[..])).unwrap();

    let mut canvas = Canvas::new();
    let scale = 40.0;

    for facet in stl.facets {
        let a = &facet.vertices[0];
        let b = &facet.vertices[1];
        let c = &facet.vertices[2];

        canvas.triangle(
            a.0 * scale,
            a.1 * scale,
            b.0 * scale,
            b.1 * scale,
            c.0 * scale,
            c.1 * scale,
        );
    }

    assert_eq!(canvas.rows().collect::<Vec<_>>(), vec![
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣀⣀⣤⣤⣤⣴⣶⣶⡾⠿⡿⣿⣶⣶⣶⣤⣤⣤⣄⣀⣀⡀",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣴⣾⣿⡿⣿⣿⣿⡿⠿⣭⣝⣲⣶⣚⣋⣏⡉⠛⠛⠻⠭⣽⡿⣛⣛⣛⠿⣿⣻⢶⣤⣀",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣠⣴⢾⣿⠟⣋⡥⠖⣯⣁⣤⣤⣴⣒⡻⠶⠭⣭⡽⠖⡗⠛⠿⠿⢷⡶⣛⣫⣽⣿⢭⣩⡟⠺⢿⣾⣽⣻⢶⣤⣀",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣴⢾⡻⣝⣞⠯⢶⣟⢾⡻⡽⠚⣏⠁⢈⣉⣭⣝⣯⣭⣵⣶⡶⡷⠶⣤⣶⣶⣞⣓⣒⣶⣉⢉⡟⠛⢷⣦⢌⣑⡪⣝⡛⠿⣿⣶⣤⡀",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⣪⡮⢾⠭⠛⣉⣤⣲⣝⢮⡥⢞⣲⣾⠿⡛⠋⠙⠛⢛⣲⣖⣪⡥⠤⡧⣤⣀⣈⣑⡺⠭⣭⡭⠛⡻⠶⣤⣤⣈⣫⡚⠽⣫⢾⣷⠦⣭⢯⡪⣦⡀",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⡿⢋⡴⣣⢕⣟⠋⠉⢀⡴⣕⣞⣋⠵⣥⠶⣷⣲⣿⣿⣿⣽⣿⣿⣿⣿⣿⣿⣶⣿⣿⣾⣿⣭⣽⣳⣧⢄⡀⠙⢟⢖⡭⣫⢖⠛⢾⣕⣅⠙⢿⣪⡻⣦⡀",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⡾⣫⢎⡴⣕⢝⢕⠏⣘⢶⠾⠓⠉⣩⣮⣷⣽⣶⣿⣿⣿⣿⣿⣿⡿⠿⠿⠟⠛⡿⣛⠿⠿⠿⣿⣿⣿⣿⣿⣻⢿⣯⣗⣶⣗⡬⡵⢯⣗⢞⠀⠙⣗⢄⡙⢝⠮⣻⣦⡀",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⡠⡾⣫⣺⠗⢡⢞⢕⣡⢚⢝⢵⢋⡣⣶⣯⣿⣿⣿⣿⡿⠟⠋⣹⣉⠶⠦⠤⣒⠶⠖⠋⡏⠑⠓⢖⣢⣤⣤⣄⣹⡙⡟⠿⣿⣿⣿⣿⣿⣽⡢⡋⠙⢗⢤⡈⢗⢝⢮⢗⢌⠻⣯⡢⡀",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⡮⣞⠟⢁⣴⠵⠋⣡⢗⣕⡵⣫⣯⣮⢟⢟⣮⣗⠋⣁⠤⡲⠛⣉⣧⠴⠞⠛⢒⣛⣛⠿⡯⢍⣛⣛⠛⠲⣤⢤⣯⣈⣙⢦⢄⡉⠛⠿⣿⢟⢯⣻⣦⣀⢳⢝⢝⢢⣑⢕⢯⡳⢌⣯⣺⣆",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⡿⢡⣏⣕⣽⣁⢀⣴⠕⠉⣰⣿⣿⣿⣷⠟⠁⡣⡒⢉⣭⠯⠛⣙⡶⠻⣗⠉⠉⡡⠒⠁⠀⡟⡄⣀⠤⠛⠉⢑⣿⠢⢌⡑⠫⢏⡚⠓⡦⡛⡕⢵⣿⣿⣿⣯⡳⣕⣕⢧⢉⣳⢵⡅⢣⡷⣻⣆",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⣿⢡⡻⣟⢾⢁⡼⣵⠧⡠⡾⣻⢿⣿⠟⠁⡠⡺⢁⢽⣍⠤⠒⠉⡰⠁⠀⢻⣵⣮⠤⠤⠔⢲⡗⠛⡤⠤⠤⣀⣼⣛⡤⠤⠬⠵⠾⢞⡟⢥⣈⢣⡀⠙⢿⣿⢿⡯⡳⡨⠗⣿⡀⠈⣿⡄⢻⢵⣻⣆",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⣳⢣⢫⣟⠎⡮⡳⡝⡧⣞⣾⣿⣕⣕⠥⡤⢚⢞⠞⢉⡟⠉⢫⡒⡴⣁⠤⠒⢹⢷⠀⠑⢄⡰⠁⡇⠀⠘⣄⠔⢺⠏⠑⡧⢄⡀⠀⣠⡟⠑⢄⠑⢽⡪⣢⡀⠙⢗⢽⣿⡿⡄⠘⡵⣄⢸⢞⣄⣯⢳⣿⣆",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⣳⣿⠃⣜⡮⢺⣱⢝⢴⣽⣿⣿⣿⠏⢩⠮⣄⠞⢁⠔⠁⡇⠀⢀⢽⢍⠀⠑⠢⣎⡘⡆⢀⠎⠑⢄⡇⡠⠔⠙⡄⡞⠀⠀⣇⡠⢬⠟⢗⠗⠤⠤⠵⢤⣗⣄⡭⢪⢹⢯⢾⣿⣿⡦⣻⢵⡱⡏⢞⣽⣣⠹⣿⣆",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣾⣳⣻⠃⣼⠟⠀⣾⠟⠁⡼⣽⣿⣿⠏⢠⡻⢠⣻⡝⠷⣤⢴⢅⣔⣁⢸⠈⠢⡀⢰⣁⢬⣻⡷⠶⠛⢛⡟⠛⠳⠶⣾⣷⢖⡉⢸⡠⠊⠀⢸⠑⢄⠀⢀⡠⡟⡅⢳⡄⣧⠈⢿⣿⣿⣟⡝⣌⢷⣹⠪⣞⡏⢧⢹⣿⡆",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢠⣿⢏⡏⣻⡿⢤⣰⡏⠀⣜⣿⣟⣿⠏⣠⢗⢯⠃⡜⡇⠀⠈⡿⣅⡀⠀⠉⡏⢑⢗⣟⠊⠁⢸⢧⠀⢀⠎⡇⠀⢠⢪⠇⠀⠉⣚⣽⢥⠤⠤⡗⠒⣒⠗⡯⣜⠀⠘⡄⢫⡧⣣⠈⢿⣿⣿⡜⣧⠙⣿⣦⠼⣾⠋⢯⣿⣧",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⢸⣷⣷⢇⣰⡿⡈⣽⣻⣿⣾⣟⣩⡃⣸⠃⡜⠀⡇⠀⡜⠀⢣⣈⣢⣄⣗⠕⠁⢹⢣⡀⢸⠈⡆⡜⠀⣇⠔⠁⡎⡠⠔⣪⠋⠀⠑⢕⢴⠥⠊⠀⢀⠜⡄⠉⠒⠼⣴⢣⠘⣧⣈⣿⣿⣿⣻⠉⠸⣇⠀⣿⡷⣺⣿⣿⠶⠤⠤⣤⣤⣤⣤⣀⣀",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣮⣧⢻⢸⢹⡇⡿⣇⣿⣿⣿⠁⣼⠉⣟⢾⢶⣶⣹⡝⠉⠉⠉⢆⢠⡳⠹⡦⣀⠈⡆⠙⣼⣤⣾⣶⣭⣯⣵⣾⢯⣄⣜⠥⠤⢒⡲⠟⠹⣳⠒⢤⠮⣀⣘⣄⣠⣤⢾⠟⣏⠙⣤⢻⣿⣿⣿⡇⢰⣿⡄⣿⣳⣬⣿⣿⣯⡭⣉⢀⠇⠈⢺⡈⣉⡿⣗⣢⢄",
        "⠀⠀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣀⣿⣇⣏⣹⣮⣻⣼⣳⣣⣻⣿⣿⡿⠀⣿⣰⠋⣇⠀⠀⡹⢻⠓⠲⠤⣼⣷⠁⠀⠘⡌⠓⢵⣴⣻⣷⣻⢯⡟⣿⢻⡿⣻⣿⣳⣤⣔⣋⣀⣀⣀⣱⣣⡮⠴⠒⠛⢹⡷⡁⠀⡇⢹⡆⡏⠸⣿⣿⣟⣷⣸⢻⡵⡏⣾⢿⢇⣿⣏⠉⠉⢹⣯⠟⠛⡿⡽⣒⢗⡪⣷⢟⣦⣄⡀",
        "⢠⣾⣿⢮⡽⢿⣿⣯⣽⡶⢿⣽⣽⣾⣿⣿⠿⠿⢿⣿⣿⣿⣿⠿⣿⡿⢿⣿⡯⢼⣿⣿⣿⡇⢸⠜⣿⢰⠉⢆⠀⡇⣀⠵⢖⠉⢸⡏⠫⣓⠢⠬⣦⣟⣿⡻⢿⣿⡟⡟⡟⡟⣷⣿⣿⣻⣽⣧⡠⠤⠒⠋⠉⣿⠑⠤⣀⠔⠁⡇⠈⢢⢱⢸⡜⣼⠀⣿⣿⣿⣿⡹⣸⣇⣧⡇⣿⣿⢺⣿⠔⠊⡹⠀⠣⡀⢣⡿⡀⡹⡟⢺⡶⢍⣿⣙⣿⣿⣿⣶⣶⣶⣶⣶⣦⣤⣀⡀",
        "⡾⣿⠔⢱⣅⡞⣿⣿⣟⣷⣜⡟⠛⠬⣿⣲⠽⢞⡷⣏⣿⣿⡯⢭⢿⠛⢵⣏⣇⡾⣿⣿⣿⡱⡝⠀⡏⡜⣀⠬⢶⠉⠀⠀⠀⠣⣿⠀⠀⠀⠑⢄⡜⣿⣿⣛⣿⠭⣪⣺⣿⣪⡪⢽⡷⢯⣿⡟⡌⠑⠒⠤⣀⢸⣇⠔⠉⠒⢄⣱⠀⠀⠙⣼⡇⠹⡄⢹⣿⣿⡏⡇⠹⣿⣏⣷⢿⠺⡏⣿⡦⢄⡇⠀⠀⠑⣼⡠⢏⠇⢣⠇⣯⢎⡰⣧⣿⣾⢟⢉⡩⠭⠛⢻⠿⣻⡿⣻⣆",
        "⣿⣿⢫⣿⠉⣟⣿⣟⡿⡿⢉⣟⢿⡻⣿⣛⠭⣉⣭⡿⣿⡟⣯⢽⣹⡭⢽⣯⡏⡏⣿⣿⣿⠉⢯⠉⣿⢏⠉⠉⢹⠛⢍⡉⢉⠝⣿⠙⠫⢍⣉⠉⢏⣿⡿⣭⣿⠭⡻⣻⣿⡻⡫⢽⣟⣻⣿⣏⠟⢍⠉⠉⠉⢹⡯⡉⠉⠉⠉⢹⡭⠝⢋⠏⡏⢉⢯⢻⣿⣿⣯⡏⠉⣿⡟⢻⡽⠉⣯⣿⡏⠉⡟⢍⡩⠝⢻⢫⠙⡯⣙⡟⡯⣉⣻⣿⡽⣿⡟⠉⠉⢉⣩⣽⣛⣯⣟⣽⡟",
        "⠸⣼⣧⣤⣷⣾⡿⣿⣯⠿⡿⢧⣤⣬⣿⣶⣯⣵⣿⣾⣿⣿⣿⣿⣧⣿⣽⣿⣷⢿⣿⣿⣿⡇⢸⢇⢿⢸⠱⡀⠀⡇⢀⠜⠣⢄⢹⡆⠀⣀⡠⠭⠺⣿⢿⣳⣿⣿⣎⣎⣏⣎⣾⣿⣭⣻⣟⡿⡦⠤⣑⡢⡀⣾⠁⢑⣤⠔⠊⡇⠘⡄⢸⢸⣇⢼⠁⣿⣿⣿⣼⠀⣸⣧⡇⣿⡏⢹⣿⣿⡠⠔⢫⠁⠑⢄⡸⡆⢑⣷⢉⣳⣧⠼⢿⣳⣿⣾⣿⣿⣿⣿⣷⣷⡿⠿⠛⠋",
        "⠀⠈⠛⠚⠚⠛⠛⠓⠚⠛⠛⠛⠚⠛⠛⠛⠛⠛⠛⠛⠛⣿⡟⣿⣻⡟⣿⢿⣺⢻⣟⣿⣿⣧⠈⡎⣾⠀⡇⠈⢢⣷⢁⣀⣠⠤⡿⣻⠛⠓⠒⠒⣒⠿⢿⣻⣿⣣⣿⣹⣏⡿⣽⣻⣿⣻⢿⢅⡈⢆⠀⠉⢹⣿⠮⢥⣀⣱⣠⠃⠀⠘⡇⣸⢻⡎⢠⣿⣿⣿⡻⣰⣹⢿⡷⣿⡡⡞⣿⡟⠑⢄⣸⠀⣀⣀⣷⢷⣷⣉⣫⣽⣿⡿⠟⠋⠉⠉⠉",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣷⡏⢹⣸⡇⢿⣿⠀⣿⣿⣿⣿⡸⢇⠘⣇⣵⡴⠷⠛⢟⠓⠢⡼⢄⣱⣣⢀⣤⣚⡡⠤⢔⠟⠿⣽⣿⠿⡿⢿⣿⡷⢿⢇⠀⡇⠙⠪⣦⢠⡳⠙⡄⠀⠀⢉⢿⢳⣶⣴⣝⡇⢸⠇⢸⣿⣿⡟⣧⡇⣿⢸⣿⡿⡿⣷⣿⡟⢋⣉⠽⡝⠛⢫⠯⢍⣦⣾⡿⠟⠉",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢹⣿⣿⠀⣿⠃⠘⣧⠀⢹⣻⣿⣿⡗⠺⣏⠹⣹⢦⢄⡀⠈⢆⠜⠀⠀⡠⢽⢝⢅⠀⠀⡰⣋⠤⠊⡏⢀⠝⡏⢁⠎⡆⢸⠈⠳⣸⠀⢀⢜⡗⠷⡒⠺⡉⢉⠏⠉⡇⢀⠎⣸⠋⡻⢒⣿⣿⣿⣳⠏⢣⣿⠛⣿⣷⣿⣺⣿⣭⠭⠶⠶⠵⠴⠾⠚⠛⠉⠁",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠸⣿⡿⡄⣹⣧⠴⣿⣎⠹⣏⢿⣿⣿⡄⠹⡢⡷⡈⢆⠈⢑⠮⣆⣔⣊⣀⡧⠤⠵⢵⢞⡋⠀⠀⡼⡰⠁⠀⡇⡜⠀⠸⣼⠀⠀⣙⣗⣕⠁⡇⠀⠈⠒⢧⡎⠀⠀⣇⠎⡰⣝⡴⠋⣽⣟⣾⣟⠏⠉⣾⠳⢤⣷⡃⡟⣼⡿",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⠹⡍⣞⡦⣸⢻⡕⢏⢞⣿⣿⣿⡄⠱⡏⢷⡈⢆⡮⠚⠁⠑⢄⢰⠁⠀⡠⢻⠉⢚⣵⣶⣯⣤⣀⣀⣟⣀⣠⣤⣿⡲⠝⢺⠁⠣⡉⢹⠑⢒⠖⢽⠽⢦⣄⢿⡲⢩⡳⠁⣼⣿⣿⢯⠏⢀⣼⡇⢀⣼⠏⣹⣻⣻⡇",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣿⣧⠹⣻⢞⡌⣧⢻⢼⡫⣿⣿⣿⡽⣴⠱⡠⠗⢟⡷⢤⠤⠤⢄⣝⣄⡼⠤⠚⡎⠁⢀⡏⢆⢀⠭⠋⡟⢍⠁⡔⠉⣏⠒⡧⣀⠀⠘⢼⠔⠁⠀⡇⢀⠔⢉⡝⠧⡼⠁⣼⣿⣿⣿⢿⢔⢽⢻⡠⣞⠏⣰⣿⣳⡟",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣿⣷⡹⡟⢞⣵⠑⢯⢇⠈⢯⣿⣾⢝⣍⠈⠳⡢⣳⢄⠑⢄⢀⡷⠋⠉⠒⠤⣇⠀⣼⣀⠜⢇⠀⠀⡇⢠⠛⢄⠀⢸⣼⢀⡠⠕⢪⢏⡳⡀⢀⡗⢁⣔⣝⡠⠮⢜⢟⢿⣿⣟⡯⣇⢮⡣⡏⣜⡾⡰⣹⣳⡟",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣷⣻⢵⡈⢾⡆⠀⢻⣆⡬⢫⡫⣷⣽⣷⣄⠈⠺⡑⠳⢄⣟⣥⣤⠤⠤⠤⢗⣻⠟⠢⠤⠬⢆⣀⣷⣁⠤⠤⠵⣶⢿⡁⠀⢠⠃⢀⡨⠝⢿⠝⢩⡪⠊⢀⣴⣷⣵⣫⡯⠪⣴⢯⠗⢹⣜⣮⡻⢱⣷⡟",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣷⣫⡷⡈⢿⢴⡚⠱⣝⢖⢯⡻⣿⣿⣿⣷⢄⢏⡢⣏⣉⡚⡤⣉⠒⠤⣸⣟⠀⢀⡠⠔⠊⢇⡇⠀⢀⡠⠊⠀⢙⣧⣠⣗⢊⡡⡴⠞⢉⡣⡋⢀⣴⣿⣿⣿⣿⠋⢀⢴⠗⠙⢲⢟⠞⡿⢡⣿⡟",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⣷⡳⡗⢥⡳⣝⢕⠳⣑⢕⢽⡙⠻⣿⡳⣕⣽⣧⣄⡉⠚⠽⣕⠛⠲⡾⠵⢯⣉⣑⣒⡒⠬⣧⣔⣓⣒⣉⣉⣥⠽⡗⢊⣩⡫⠝⠋⣑⡶⣗⣝⡵⡿⡿⣫⢗⢞⡵⠃⣠⢴⠟⢁⣝⡧⣿⡟",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠪⡻⣦⡙⢕⡵⣕⢜⡍⠳⢕⣄⠈⡫⣻⢿⣿⣿⣿⣷⣦⣌⣏⢺⠓⠶⠶⠾⣛⣍⣉⠉⡏⣉⣭⣕⡪⠭⣤⡔⢻⠋⣁⣤⣶⣿⣷⣿⡾⣿⡯⡚⣹⢕⢕⡱⢓⢝⡵⢁⣴⡻⣫⡮⠊",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⣿⡢⣝⢌⠓⢝⣆⠀⢙⣝⡷⣬⠯⢟⣿⣟⡿⣿⣽⣻⣿⣷⣶⣶⣦⣤⣤⣕⣫⣏⣁⣤⣤⣤⣶⣶⣾⣿⣿⣿⣿⣿⢿⡿⡿⠋⢁⣠⣽⣕⠋⣝⢔⢕⢯⠖⡵⣫⡾⠋",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⣮⡻⣳⣄⠙⢗⢾⣄⣙⡵⡫⢖⣕⣄⠉⠛⠽⣿⣻⠿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢿⣿⣿⣿⣻⣯⡵⢮⠕⣛⣞⢯⠗⠉⠀⣘⣞⠵⣫⠗⣡⣾⡾⠋",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠻⡮⡻⡵⠧⣽⣾⡵⣪⢍⡳⡚⠻⠷⢦⣤⣃⡩⠿⠯⢭⣚⠛⠓⠺⠯⡯⠭⠿⣛⣿⣛⣉⣉⠁⣀⣣⣴⣷⣛⡭⠶⣕⢶⡺⠟⢋⡡⢵⡥⡾⡫⠋",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠺⢿⣷⣦⣍⣚⠯⣛⠚⠽⣶⣄⣀⡞⠙⢻⣟⣛⣛⣶⣿⡶⢦⣤⣧⣶⣿⠶⠶⣞⠿⠟⠛⠉⠙⣏⣠⢮⣲⣝⣾⡥⣔⣞⢯⣻⣽⠾⠊",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⠻⢿⣽⣲⢿⣶⣤⣉⡿⠻⢽⣷⠶⣒⣫⣶⣤⣤⣄⣀⣇⣤⠶⠯⢭⣭⣛⣻⠿⠶⠟⠻⣎⡭⠟⣊⣵⣾⣽⠾⠛⠉",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⠻⢾⣽⣺⣯⣝⣛⣛⣫⣿⠧⢤⣉⣉⡉⠓⡗⣚⣻⣿⣓⠾⢧⣤⣶⣶⣶⣯⣷⣶⣿⠿⠛⠉",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠉⠛⠛⠛⠿⠿⠿⢿⣿⣿⣿⣯⣯⣭⣶⣶⣾⠿⠿⠿⠟⠛⠛⠋⠉⠉",
        "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠉⠉",
    ]);
}