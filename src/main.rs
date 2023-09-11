use flatgeobuf::*;
use std::error::Error;
use std::io::BufReader;
use std::fs::File;
use std::vec::Vec;
use std::collections::HashMap;
use rand::prelude::*;

trait FisherYates {
    fn shuffle(&self, rng: &ThreadRng); 
}

struct Coord {
    x: f64,
    y: f64
}

struct Polygon {
    properties: HashMap<String, String>,
    vertices: Vec<Coord>,
}

impl FisherYates for Polygon {
    fn shuffle(&self, rng: &ThreadRng) {
        for i in (1..self.vertices.len()).rev() {
            let j: usize = rng.gen_range(0..=i);  
            self.vertices.swap(i, j);
        }
    }
}

struct Segment {
    v_0: Coord,
    v_1: Coord,
    is_inserted: bool,    
}

struct Trapezoid {
    left_seg: Segment,
    right_seg: Segment,
    high_coord: Coord,
    low_coord: Coord,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut filein = BufReader::new(File::open("mo_senate_districts.fgb")?);
    let mut fgb = FgbReader::open(&mut filein)?.select_all()?;
    let mut map_polygons: Vec<Polygon> = Vec::new(); 

    while let Some(feature) = fgb.next()? {
        let geom = feature.geometry();
        let xy = geom.unwrap().xy().unwrap();
        if xy.len() % 16 != 0 {
            panic!("bytes aren't correct")
        }
        let props = feature.properties()?;
        let mut verts: Vec<Coord> = Vec::new();
        let mut ind: usize = 0;
        while ind < xy.len() {
            verts.push(Coord {
                x: xy.get(ind),
                y: xy.get(ind+1),
            });    
            ind += 2;
        }
        map_polygons.push(Polygon {
            properties: props,
            vertices: verts,
        });
    }
    
    for poly in map_polygons {
        println!("{} {} {}", poly.properties.get("Name").unwrap(), poly.vertices[0].x, poly.vertices[0].y);
    }
    Ok(())
}


fn triangulate(map_polygons: &Vec<Polygon>) -> Vec<f64> {
    let mut rng = thread_rng();
    let triangles_vec: Vec<f64> = Vec::from([]);
    if map_polygons.len() == 0 {
        triangles_vec
    }
    else {
       for poly in map_polygons.iter() {
           poly.shuffle(&rng);
       }

    }
}

fn shuffle(mut coords: &Vec<Coord>, mut rng: &ThreadRng) {
    
}
