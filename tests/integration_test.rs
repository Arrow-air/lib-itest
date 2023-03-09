//! Hello world example for Rust.
//! This is a crate description, needed or else missing_docs warning will occur.

use lib_itest::{
    self,
    job::{
        Job,
        Step
    },
    arrow::assets,
    // arrow::cargo,
    arrow::db
};

#[test]
fn r2_soft_demo() {
    let mut job = Job {
        name: "R2 Soft Demo".into(),
        description: "Testing".to_string(),
        requirements: vec![],
        setup: vec![
            db::DatabaseResetStep {},
    
            //
            // Create known aircraft
            //
            assets::CreateAircraftStep{
                id: "N12345".into(),
                name: "Marauder".into(),
                owner: "The Bad Batch".into(),
                model: "Omicron".into(),
                manufacturer: "Cygnus Spaceworks".into(),
                icao: "SW123".into(),
            },
            assets::CreateAircraftStep{
                id: "N12345".into(),
                name: "Serenity".into(),
                owner: "M. Reynolds".into(),
                model: "Firefly".into(),
                manufacturer: "Osiris Shipworks".into(),
                icao: "F2002".into(),
            },
    
            //
            // Create Known Vertiports
            //
            assets::CreateVertiportStep {
                name: "Balboa Park West".into(),
                owner: "San Diego Air & Space Museum".into(),
                latitude: 32.726296391742515,
                longitude: -117.15431204941152
            },
    
            assets::CreateVertiportStep {
                name: "Embarcadero".into(),
                owner: "San Diego Convention Center".into(),
                latitude: 32.70449008593309,
                longitude: -117.16103154557862
            },
    
            assets::CreateVertiportStep {
                name: "Liberty Station".into(),
                owner: "City of San Diego".into(),
                latitude: 32.737756678402015,
                longitude: -117.20918603251332
            },
    
            //
            // Add One Pad Per Vertiport
            //
        ],
        steps: vec![],
        cleanup: vec![]
    };

    job.execute();
}
