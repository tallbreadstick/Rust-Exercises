use rstest::rstest;
use crate::task::Marketplace;

#[rstest]
pub fn timber_sell_orders() {
    let mut marketplace = Marketplace::init();
    let timber = vec![
        (500, 25),
        (1200, 24),
        (350, 25),
        (400, 20),
        (800, 20)
    ];
    for (quantity, price) in timber {
        marketplace.sell_order("Timber", quantity, price);
    }
    assert_eq!(marketplace.buy("Timber", 800), Some(18000));
    assert_eq!(marketplace.buy("Timber", 800), Some(18250));
    assert_eq!(marketplace.buy("Timber", 800), Some(17400));
    assert_eq!(marketplace.buy("Timber", 800), Some(19200));
    assert_eq!(marketplace.buy("Timber", 800), None);
}

#[rstest]
pub fn stone_sell_orders() {
    let mut marketplace = Marketplace::init();
    let stone = vec![
        (800, 35),
        (1000, 33),
        (400, 33),
        (2000, 32),
        (100, 25)
    ];
    for (quantity, price) in stone {
        marketplace.sell_order("Stone", quantity, price);
    }
    assert_eq!(marketplace.buy("Stone", 800), Some(26200));
    assert_eq!(marketplace.buy("Stone", 800), Some(27400));
    assert_eq!(marketplace.buy("Stone", 800), Some(26300));
    assert_eq!(marketplace.buy("Stone", 800), Some(25600));
    assert_eq!(marketplace.buy("Stone", 800), Some(25600));
}

#[rstest]
pub fn copper_sell_orders() {
    let mut marketplace = Marketplace::init();
    let copper = vec![
        (470, 48),
        (320, 49),
        (600, 45),
        (120, 47),
        (200, 46),
        (100, 47)
    ];
    for (quantity, price) in copper {
        marketplace.sell_order("Copper", quantity, price);
    }
    assert_eq!(marketplace.buy("Copper", 800), Some(38100));
    assert_eq!(marketplace.buy("Copper", 800), Some(37230));
    assert_eq!(marketplace.buy("Copper", 800), None);
    assert_eq!(marketplace.buy("Copper", 800), None);
    assert_eq!(marketplace.buy("Copper", 800), None);
}

#[rstest]
pub fn timber_buy_orders() {
    let mut marketplace = Marketplace::init();
    let timber = vec![
        (500, 25),
        (1200, 24),
        (350, 25),
        (400, 20),
        (800, 20),
    ];
    for (quantity, price) in timber {
        marketplace.buy_order("Timber", quantity, price);
    }
    assert_eq!(marketplace.sell("Timber", 800), Some(19200));
    assert_eq!(marketplace.sell("Timber", 800), Some(16000));
    assert_eq!(marketplace.sell("Timber", 800), Some(19700));
    assert_eq!(marketplace.sell("Timber", 800), Some(17950));
    assert_eq!(marketplace.sell("Timber", 800), None);
}

#[rstest]
pub fn stone_buy_orders() {
    let mut marketplace = Marketplace::init();
    let stone = vec![
        (800, 35),
        (1000, 33),
        (400, 33),
        (2000, 32),
        (100, 25),
    ];
    for (quantity, price) in stone {
        marketplace.buy_order("Stone", quantity, price);
    }
    assert_eq!(marketplace.sell("Stone", 800), Some(25600));
    assert_eq!(marketplace.sell("Stone", 800), Some(25600));
    assert_eq!(marketplace.sell("Stone", 800), Some(26400));
    assert_eq!(marketplace.sell("Stone", 800), Some(28000));
    assert_eq!(marketplace.sell("Stone", 800), Some(26000));
}

#[rstest]
pub fn copper_buy_orders() {
    let mut marketplace = Marketplace::init();
    let copper = vec![
        (470, 48),
        (320, 49),
        (600, 45),
        (120, 47),
        (200, 46),
        (100, 47),
    ];
    for (quantity, price) in copper {
        marketplace.buy_order("Copper", quantity, price);
    }
    assert_eq!(marketplace.sell("Copper", 800), Some(36600));
    assert_eq!(marketplace.sell("Copper", 800), Some(38310));
    assert_eq!(marketplace.sell("Copper", 800), None);
    assert_eq!(marketplace.sell("Copper", 800), None);
    assert_eq!(marketplace.sell("Copper", 800), None);
}
