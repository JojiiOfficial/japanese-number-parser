use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref DIGITS: HashMap<char, char> = {
        let mut m = HashMap::with_capacity(44);
        m.insert('〇', '0');
        m.insert('零', '0');
        m.insert('0', '0');
        m.insert('０', '0');
        m.insert('一', '1');
        m.insert('壱', '1');
        m.insert('壹', '1');
        m.insert('1', '1');
        m.insert('１', '1');
        m.insert('二', '2');
        m.insert('弐', '2');
        m.insert('貳', '2');
        m.insert('2', '2');
        m.insert('２', '2');
        m.insert('三', '3');
        m.insert('参', '3');
        m.insert('參', '3');
        m.insert('3', '3');
        m.insert('３', '3');
        m.insert('四', '4');
        m.insert('肆', '4');
        m.insert('4', '4');
        m.insert('４', '4');
        m.insert('五', '5');
        m.insert('伍', '5');
        m.insert('5', '5');
        m.insert('５', '5');
        m.insert('六', '6');
        m.insert('陸', '6');
        m.insert('6', '6');
        m.insert('６', '6');
        m.insert('七', '7');
        m.insert('漆', '7');
        m.insert('柒', '7');
        m.insert('7', '7');
        m.insert('７', '7');
        m.insert('八', '8');
        m.insert('捌', '8');
        m.insert('8', '8');
        m.insert('８', '8');
        m.insert('九', '9');
        m.insert('玖', '9');
        m.insert('9', '9');
        m.insert('９', '9');
        m
    };
    pub static ref IN_GROUP_POWERS: HashMap<char, u32> = {
        let mut m = HashMap::with_capacity(7);
        m.insert('十', 1);
        m.insert('拾', 1);
        m.insert('百', 2);
        m.insert('佰', 2);
        m.insert('千', 3);
        m.insert('仟', 3);
        m.insert('阡', 3);
        m
    };
    pub static ref SEPARATOR_POWERS: HashMap<&'static str, u32> = {
        let mut m = HashMap::with_capacity(20);
        m.insert("万", 4);
        m.insert("萬", 4);
        m.insert("億", 8);
        m.insert("兆", 12);
        m.insert("京", 16);
        m.insert("垓", 20);
        m.insert("𥝱", 24);
        m.insert("秭", 24);
        m.insert("穣", 28);
        m.insert("溝", 32);
        m.insert("澗", 36);
        m.insert("正", 40);
        m.insert("載", 44);
        m.insert("極", 48);
        m.insert("恒河沙", 52);
        m.insert("阿僧祇", 56);
        m.insert("那由他", 60);
        m.insert("那由多", 60);
        m.insert("不可思議", 64);
        m.insert("無量大数", 68);
        m
    };
    pub static ref ALTERNATE_LARGE_POWERS: HashMap<&'static str, u32> = {
        let mut m = HashMap::with_capacity(5);
        m.insert("恒河沙", 56);
        m.insert("阿僧祇", 64);
        m.insert("那由他", 72);
        m.insert("那由多", 72);
        m.insert("不可思議", 80);
        m.insert("無量大数", 88);
        m
    };
    pub static ref FINANCIAL_SEPARATORS: HashMap<&'static str, u32> = {
        let mut m = HashMap::with_capacity(12);
        m.insert("千", 3);
        m.insert("仟", 3);
        m.insert("阡", 3);
        m.insert("百万", 6);
        m.insert("佰万", 6);
        m.insert("百萬", 6);
        m.insert("佰萬", 6);
        m.insert("十億", 9);
        m.insert("拾億", 9);
        m.insert("兆", 12);
        m.insert("千兆", 15);
        m.insert("仟兆", 15);
        m
    };
    pub static ref BU_FRACTIONALS: HashMap<char, u32> = {
        let mut m = HashMap::with_capacity(10);
        m.insert('分', 1);
        m.insert('厘', 2);
        m.insert('毛', 3);
        m.insert('糸', 4);
        m.insert('忽', 5);
        m.insert('微', 6);
        m.insert('繊', 7);
        m.insert('沙', 8);
        m.insert('塵', 9);
        m.insert('埃', 10);
        m
    };
    pub static ref WARI_FRACTIONALS: HashMap<char, u32> = {
        let mut m = HashMap::with_capacity(11);
        m.insert('割', 1);
        m.insert('分', 2);
        m.insert('厘', 3);
        m.insert('毛', 4);
        m.insert('糸', 5);
        m.insert('忽', 6);
        m.insert('微', 7);
        m.insert('繊', 8);
        m.insert('沙', 9);
        m.insert('塵', 10);
        m.insert('埃', 11);
        m
    };
}

pub const SEPARATORS: [char; 3] = ['、', '，', ','];
pub const DECIMAL_POINTS: [char; 6] = ['.', '．', '・', '･', '點', '点'];
