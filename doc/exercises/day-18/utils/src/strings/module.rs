pub mod conversion {
    pub fn reverse_string(string: &str) -> String {
        let mut reversed = String::new();

        for s in string.chars().rev() {
            reversed.push(s);
        }
        reversed
    }

    pub fn upper_string(string: &str) -> String {
        let mut upper = String::new();

        for s in string.chars() {
            upper.push_str(&s.to_uppercase().to_string());
        }
        upper
    }

    pub fn rotate_string_r(string: &str) -> String {
        let mut rotated_vec: Vec<char> = string.chars().collect();

        for (i, s) in string.chars().enumerate() {
            rotated_vec[(i + 1) % string.len()] = s;
        }
        let result: String = rotated_vec.into_iter().collect();

        result
    }

    pub fn rotate_string_l(string: &str) -> String {
        let mut rotated_vec: Vec<char> = string.chars().collect();

        for (i, s) in string.chars().enumerate() {
            let x: i16 = i as i16;

            if x - 1 < 0 {
                rotated_vec
                    [(x + string.len() as i16 - 1).rem_euclid(string.len() as i16) as usize] = s;
            } else {
                rotated_vec[(x - 1).rem_euclid(string.len() as i16) as usize] = s;
            }
        }
        let result: String = rotated_vec.into_iter().collect();

        result
    }

    pub fn rotate_string_r_by(string: &str, rot: i16) -> String {
        let mut rotated_vec: Vec<char> = string.chars().collect();

        for (i, s) in string.chars().enumerate() {
            let x: i16 = i as i16;

            rotated_vec[(x + rot).rem_euclid(string.len() as i16) as usize] = s;
        }
        let result: String = rotated_vec.into_iter().collect();

        result
    }

    pub fn rotate_string_l_by(string: &str, rot: i16) -> String {
        let mut rotated_vec: Vec<char> = string.chars().collect();

        for (i, s) in string.chars().enumerate() {
            let x: i16 = i as i16;

            if x - rot < 0 {
                rotated_vec
                    [(x + string.len() as i16 - rot).rem_euclid(string.len() as i16) as usize] = s;
            } else {
                rotated_vec[(x - rot).rem_euclid(string.len() as i16) as usize] = s;
            }
        }
        let result: String = rotated_vec.into_iter().collect();

        result
    }
}
