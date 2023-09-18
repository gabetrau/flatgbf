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
    let mut map_polygons: Vec<Polygon<f32>> = Vec::new(); 
    let origin_long: f32 = 38.749130;
    let origin_lat: f32 = -92.529521;
    while let Some(feature) = fgb.next()? {
        let geom = feature.geometry();
        let xy = geom.unwrap().xy().unwrap();
        if xy.len() % 2 != 0 {
            panic!("bytes aren't correct")
        }
        let properties = feature.properties()?;
        let mut ind: usize = 0;
        let mut coords: Vec<Coord<f32>> = Vec::new();
        if xy.len() <= 6 {
            panic!("not enough vertices")
        }
        while ind < xy.len() {
            coords.push(coord! {
                x: (xy.get(ind) as f32 - origin_lat) / 3.0,
                y: (xy.get(ind+1) as f32 - origin_long) / 3.0,
            });
            ind += 2;
        }
        
        let line_string: LineString<f32> = coords.into_iter().collect();
        
        map_polygons.push(Polygon::new(
                line_string,
                vec![],
        ));
    }

    for mp in map_polygons.iter() {
        let triangles: Vec<Triangle<f32>> = mp.earcut_triangles();
        let tri = triangles.get(0).unwrap();
    }

    let shape = map_polygons.get(map_polygons.len() - 1).unwrap().earcut_triangles();
    let mut counter: usize = 0;
    for triang in shape.iter() {
        println!("{},{},0.0,", triang.0.x, triang.0.y);
        println!("{},{},0.0,", triang.1.x, triang.1.y);
        println!("{},{},0.0,", triang.2.x, triang.2.y);
        counter += 9;
    }
    println!("\n\n{}", counter);
    Ok(())
}

