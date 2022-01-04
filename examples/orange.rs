use iptrie::*;
use std::net::Ipv4Addr;

fn main() {

    let prefixes = [
        "45.56.0.0/18|16591",
    "45.56.64.0/20|63949",
    "45.56.112.0/21|63949",
    "45.56.120.0/21|63949",
    "45.56.132.0/22|62874",
   "45.56.138.0/23|8100",
    "45.56.144.0/23|62874",
    "45.56.146.0/24|206092",
    "45.57.0.0/17|2906",
    "45.57.8.0/23|40027",
    "45.57.8.0/24|40027",
    "45.57.9.0/24|40027",
    "45.57.40.0/23|40027",
    "45.57.40.0/24|40027",
    "45.57.41.0/24|40027",
    "45.57.44.0/24|2906",
    "45.57.45.0/24|2906",
    "45.57.56.0/24|2906",
    "45.57.60.0/24|2906",
    "45.57.63.0/24|2906",
    "45.57.68.0/24|2906",
    "45.57.69.0/24|2906",
    "45.57.76.0/23|40027",
    "45.57.77.0/24|40027",
    "45.57.90.0/23|40027",
    "45.57.90.0/24|40027",
    "45.57.91.0/24|40027",
    "45.57.128.0/23|55286",
    ];
    let mut trie = IpPrefixMap::with_root(20);

    prefixes.iter()
        .for_each(|x| {
            let mut x = x.split('|');
            let p = x.next().unwrap().parse::<IpWholePrefix<Ipv4>>().unwrap();
            let a = x.next().unwrap().parse::<u32>().unwrap();
            trie.insert(p,a);
        });


    #[cfg(feature = "graphviz")]
    trie.open_dot_view().expect("can’t open dot view");

    let trie = trie.compile();
    #[cfg(feature = "graphviz")]
    trie.open_dot_view().expect("can’t open dot view");

}