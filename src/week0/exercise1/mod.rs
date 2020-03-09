pub fn overlap_rectangles(rect1: Vec<i32>, rect2: Vec<i32>) -> bool {
    if rect1[0] >= rect2[2] || rect1[2] <= rect2[0] || rect1[1] >= rect2[3] || rect1[3] <= rect2[1]{
        return false;
    }
    true
}

pub fn overlap_rectangles_2(rect1: Vec<i64>, rect2: Vec<i64>) -> bool {
    let mut izq;
    let mut der;
    let mut bot;
    let mut up;

    if rect1 == rect2 {
        return true;
    }
    if rect1[0] <= rect2[0] {
        izq = rect1.clone();
        der = rect2.clone();
    } else {
        izq = rect2.clone();
        der = rect1.clone();
    }
    if rect1[1] <= rect2[1] {
        bot = rect1.clone();
        up = rect2.clone();
    } else {
        bot = rect2.clone();
        up = rect1.clone();
    }

    if (der[0] >= izq[0] && der[0] < izq[2]) && (up[1] >= bot[1] && up[1] < bot[3]) {
        return true;
    }
    false
}
