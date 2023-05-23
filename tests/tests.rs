#[cfg(test)]
mod tests {
    
    use qmidi::qsong_orig::create_qsong_orig;
    use qmidi::qsong::*;
    use midi_file::core::Velocity;

    #[test]
    fn test_create_song() {
        let mut pattern = Vec::new();
        pattern.push(PatternStep { is_rest: false, duration: HALF, velocity: Velocity::new(60) });
        pattern.push(PatternStep { is_rest: false, duration: QUARTER, velocity: Velocity::new(70)  });
        pattern.push(PatternStep { is_rest: false, duration: QUARTER, velocity: Velocity::new(80)  });

        pattern.push(PatternStep { is_rest: false, duration: HALF, velocity: Velocity::new(60)  });
        pattern.push(PatternStep { is_rest: false, duration: HALF, velocity: Velocity::new(60)  });

        pattern.push(PatternStep { is_rest: false, duration: QUARTER, velocity: Velocity::new(64)  });
        pattern.push(PatternStep { is_rest: false, duration: QUARTER, velocity: Velocity::new(75)  });
        pattern.push(PatternStep { is_rest: false, duration: QUARTER, velocity: Velocity::new(82)  });
        pattern.push(PatternStep { is_rest: true, duration: QUARTER, velocity: Velocity::new(82)  });

        create_qsong("input/20230519_Songs_M_8_1.csv", "output/20230519_Songs_P_M_8_1.mid", 90, 4, &pattern);
        create_qsong("input/20230519_Songs_M_8_2.csv", "output/20230519_Songs_P_M_8_2.mid", 90, 4, &pattern);
        create_qsong("input/20230519_Songs_G_8_1.csv", "output/20230519_Songs_P_G_8_1.mid", 90, 4, &pattern);
        create_qsong("input/20230519_Songs_G_8_2.csv", "output/20230519_Songs_P_G_8_2.mid", 90, 4, &pattern);
        create_qsong("input/20230519_Songs_M_20_1.csv", "output/20230519_Songs_P_M_20_1.mid", 90, 4, &pattern);
        create_qsong("input/20230519_Songs_M_20_2.csv", "output/20230519_Songs_P_M_20_2.mid", 90, 4, &pattern);
        create_qsong("input/20230519_Songs_M_20_3.csv", "output/20230519_Songs_P_M_20_3.mid", 90, 4, &pattern);
        create_qsong("input/20230519_Songs_M_20_4.csv", "output/20230519_Songs_P_M_20_4.mid", 90, 4, &pattern);
   }    

    #[test]
    fn test_create_song_orig() {
        create_qsong_orig("", "output/test1.mid");
   }    
}