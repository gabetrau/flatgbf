use flatgeobuf::*;
use std::error::Error;
use std::io::BufReader;
use std::fs::File;
use std::vec::Vec;
use geo::*;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn Error>> {
    let mut filein = BufReader::new(File::open("mo_senate_districts.fgb")?);
    let mut fgb = FgbReader::open(&mut filein)?.select_all()?;
    let mut map_polygons: Vec<Polygon> = Vec::new(); 

    while let Some(feature) = fgb.next()? {
        let geom = feature.geometry();
        let xy = geom.unwrap().xy().unwrap();
        if xy.len() % 2 != 0 {
            panic!("bytes aren't correct")
        }
        let properties = feature.properties()?;
        let mut ind: usize = 0;
        let mut coords: Vec<Coord> = Vec::new();
        if xy.len() <= 6 {
            panic!("not enough vertices")
        }
        while ind < xy.len() {
            coords.push(coord! {
                x: xy.get(ind),
                y: xy.get(ind+1),
            });
            ind += 2;
        }
        
        let line_string: LineString<f64> = coords.into_iter().collect();
        
        map_polygons.push(Polygon::new(
                line_string,
                vec![],
        ));
    }

    for mp in map_polygons.iter() {
        let triangles: Vec<Triangle> = mp.earcut_triangles();
        let tri = triangles.get(0).unwrap();
        println!("c1 [ x: {}, y: {} ]\nc2: [ x: {}, y: {} ]\nc3: [ x: {}, y: {} ]\n", tri.0.x, tri.0.y, tri.1.x, tri.1.y, tri.2.x, tri.2.y); 
    }

    Ok(())
}




