use flatgeobuf::*;
use std::error::Error;
use std::io::BufReader;
use std::fs::File;
use std::vec::Vec;
use std::collections::HashMap;
use rand::prelude::*;

struct Vertex {
    x: f64,
    y: f64,
    is_inserted: bool,
}

struct Polygon {
    properties: HashMap<String, String>,
    edges: Vec<Edge>,
}

struct Edge {
    v_0: Vertex,
    v_1: Vertex, 
}

trait FisherYates {
    fn shuffle(&self, rng: &ThreadRng); 
}

impl FisherYates for Polygon {
    fn shuffle(&self, rng: &ThreadRng) {
        for i in (1..self.edges.len()).rev() {
            let j: usize = rng.gen_range(0..=i);  
            self.edges.swap(i, j);
        }
    }
}

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
        let mut edges: Vec<Edge> = Vec::new();
        let mut ind: usize = 0;
        if xy.len() <= 6 {
            panic!("not enough vertices")
        }
        while ind < xy.len() {
            if ind == 0 {
                ind += 2; // each entry is either x or y starting with x and alternating
            }
            else if ind == xy.len() - 2  {
                if xy.get(ind) == xy.get(0) && xy.get(ind+1) == xy.get(1) {
                    // point back to first vertex depends on how file is formated
                    edges.push(Edge {
                        v_0: Vertex {
                            x: xy.get(ind),
                            y: xy.get(ind+1),
                            is_inserted:false,
                        },
                        v_1: Vertex {
                            x: xy.get(0),
                            y: xy.get(1),
                            is_inserted:false,
                        },
                    });
                }
            }
            else {
                edges.push(Edge {
                    v_0: Vertex {
                        x: xy.get(ind),
                        y: xy.get(ind+1),
                        is_inserted:false,
                    },
                    v_1: Vertex {
                        x: xy.get(ind),
                        y: xy.get(ind+1),
                        is_inserted:false,
                    },
                });
            }
        }
        map_polygons.push(Polygon {
            properties,
            edges,
        });
    }

    Ok(())
}



fn triangulate(map_polygons: &Vec<Polygon>) -> Vec<f64> {
    let mut rng = thread_rng();
    let triangles_vec: Vec<f64> = Vec::from([]);
    if map_polygons.len() == 0 {
        triangles_vec
    }




}


