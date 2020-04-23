pub mod stroke;
pub mod sketch;
pub mod boundingbox;
pub mod serialization;

#[cfg(test)]
mod tests {
    use serde_json::json;
    use tempfile::tempdir;

    use crate::boundingbox::BoundingBox;
    use crate::serialization::json_serializer;
    use crate::sketch::Sketch;
    use crate::stroke::Stroke;
    use crate::stroke::StrokeBuilder;

    extern crate serde_json;

    #[test]
    fn stroke() {
        let x = vec![10., 20., 30., 40., 50.];
        let y = vec![1., 2., 3., 4., 5.];
        let timestamp = vec![1, 2, 3, 4, 5];
        let pressure = vec![1., 2., 3., 4., 5.];

        let stroke = Stroke::new(x, y, timestamp, pressure);

        assert_eq!(stroke.x_min(), 10.);
        assert_eq!(stroke.x_max(), 50.);
        assert_eq!(stroke.y_min(), 1.);
        assert_eq!(stroke.y_max(), 5.);
        assert_eq!(stroke.timestamp_min(), 1);
        assert_eq!(stroke.timestamp_max(), 5);
        assert_eq!(stroke.pressure_min(), 1.);
        assert_eq!(stroke.pressure_max(), 5.);
        assert_eq!(stroke.len(), 5);
    }

    #[test]
    fn bounding_box_constructor() {
        BoundingBox::new(0., 0., 1., 1.);
    }

    #[test]
    #[should_panic]
    fn bounding_box_constructor_switched_bounds() {
        BoundingBox::new(2., 1., 1., 0.);
    }

    #[test]
    #[should_panic]
    fn bounding_box_constructor_switched_bounds_2() {
        BoundingBox::new(0., 1., 0., -1.);
    }

    #[test]
    #[should_panic]
    fn bounding_box_constructor_switched_bounds_3() {
        BoundingBox::new(0., 2., 1., 1.);
    }

    #[test]
    #[should_panic]
    fn bounding_box_constructor_switched_bounds_4() {
        BoundingBox::new(-1., 1., 0., 0.);
    }

    #[test]
    #[should_panic]
    fn bounding_box_constructor_switched_bounds_5() {
        BoundingBox::new(1., 0., 0., 1.);
    }

    #[test]
    fn bounding_box_merge() {
        let bb1 = BoundingBox::new(0., 0., 1., 1.);
        let bb1a = BoundingBox::new(0.5, 0.5, 1.5, 1.5);
        let bb2 = BoundingBox::new(-2., -2., -1., -1.);

        let bb1_bb1a = bb1.merge(&bb1a);

        assert_eq!(bb1_bb1a, bb1a.merge(&bb1));
        assert_eq!(bb1_bb1a.x_min, 0.);
        assert_eq!(bb1_bb1a.x_max, 1.5);
        assert_eq!(bb1_bb1a.y_min, 0.);
        assert_eq!(bb1_bb1a.y_max, 1.5);
        assert_eq!(bb1_bb1a.width, 1.5);
        assert_eq!(bb1_bb1a.height, 1.5);

        let bb2_bb1 = bb1.merge(&bb2);

        assert_eq!(bb2_bb1, bb2.merge(&bb1));
        assert_eq!(bb2_bb1.x_min, -2.);
        assert_eq!(bb2_bb1.x_max, 1.);
        assert_eq!(bb2_bb1.y_min, -2.);
        assert_eq!(bb2_bb1.y_max, 1.);
        assert_eq!(bb2_bb1.width, 3.);
        assert_eq!(bb2_bb1.height, 3.);
    }

    #[test]
    fn bounding_box_intersects() {
        let bb1 = BoundingBox::new(0., 0., 1., 1.);
        let bb1a = BoundingBox::new(0.5, 0.5, 1.5, 1.5);
        let bb2 = BoundingBox::new(-2., -2., -1., -1.);

        let bb1_intersects_bb2 = bb1.intersects(&bb2);
        let bb2_intersects_bb1 = bb2.intersects(&bb1);

        assert_eq!(bb2_intersects_bb1, bb1_intersects_bb2);
        assert!(!bb2_intersects_bb1);

        let bb3 = BoundingBox::new(0.25, 0.25, 0.75, 0.75);
        let bb1_intersects_bb3 = bb1.intersects(&bb3);
        let bb3_intersects_bb1 = bb3.intersects(&bb1);

        assert_eq!(bb1_intersects_bb3, bb3_intersects_bb1);
        assert!(bb1_intersects_bb3);
        assert!(!bb2.intersects(&bb3));
        assert!(bb1.intersects(&bb1a));
        assert!(bb1a.intersects(&bb1));
    }

    #[test]
    fn bounding_box_get_intersection() {
        let a = BoundingBox::new(0., 0., 2., 2.);
        let b = BoundingBox::new(1., 1., 3., 3.);

        let a_b = a.get_intersection(&b).unwrap();
        assert_eq!(a_b, b.get_intersection(&a).unwrap());
        assert_eq!(a_b.x_min, 1.);
        assert_eq!(a_b.x_max, 2.);
        assert_eq!(a_b.y_min, 1.);
        assert_eq!(a_b.y_max, 2.);
        assert_eq!(a_b.width, 1.);
        assert_eq!(a_b.height, 1.);

        let c = BoundingBox::new(0., 0., 2., 1.);
        let d = BoundingBox::new(1., -1., 3., 2.);

        let c_d = c.get_intersection(&d).unwrap();
        assert_eq!(c_d, d.get_intersection(&c).unwrap());
        assert_eq!(c_d.x_min, 1.);
        assert_eq!(c_d.y_min, 0.);
        assert_eq!(c_d.x_max, 2.);
        assert_eq!(c_d.y_max, 1.);
        assert_eq!(c_d.width, 1.);
        assert_eq!(c_d.height, 1.);

        let e = BoundingBox::new(0., 0., 1., 1.);
        let f = BoundingBox::new(2., 2., 3., 3.);

        let e_f = e.get_intersection(&f);
        assert_eq!(e_f, f.get_intersection(&e));
        assert_eq!(e_f, None);

        let g = BoundingBox::new(0., 0., 3., 3.);
        let h = BoundingBox::new(1., 1., 2., 2.);

        let g_h = g.get_intersection(&h).unwrap();
        assert_eq!(g_h, h.get_intersection(&g).unwrap());
        assert_eq!(g_h.x_min, 1.);
        assert_eq!(g_h.y_min, 1.);
        assert_eq!(g_h.x_max, 2.);
        assert_eq!(g_h.y_max, 2.);
        assert_eq!(g_h.width, 1.);
        assert_eq!(g_h.height, 1.);

        let i = BoundingBox::new(0., 0., 2., 1.);
        let j = BoundingBox::new(1., 0., 3., 1.);

        let i_j = i.get_intersection(&j).unwrap();
        assert_eq!(i_j, j.get_intersection(&i).unwrap());
        assert_eq!(i_j.x_min, 1.);
        assert_eq!(i_j.y_min, 0.);
        assert_eq!(i_j.x_max, 2.);
        assert_eq!(i_j.y_max, 1.);
        assert_eq!(i_j.width, 1.);
        assert_eq!(i_j.height, 1.);
    }

    #[test]
    fn sketch() {
        let x = vec![1., 2., 3., 4., 5., 32.];
        let y = vec![1., 2., 3., 4., 5.];
        let timestamp = vec![1, 2, 3, 4];
        let pressure = vec![1., 2., 3., 4.];

        let stroke = Stroke::new(x, y, timestamp, pressure);

        let strokes = vec![stroke.clone()];
        let mut sketch = Sketch::new(strokes);
        assert_eq!(sketch.len(), 1);

        sketch.add_stroke(stroke);
        assert_eq!(sketch.len(), 2);
    }

    #[test]
    fn stroke_builder() {
        let mut stroke_builder = StrokeBuilder::new();
        stroke_builder.add_point(1., 2., 3, 4.);
        stroke_builder.add_point(2., 3., 4, 5.);
        let stroke = stroke_builder.build();

        assert_eq!(stroke.len(), 2);
    }

    #[test]
    fn meta() {
        let x = vec![1., 2., 3., 4., 5., 32.];
        let y = vec![1., 2., 3., 4., 5.];
        let timestamp = vec![1, 2, 3, 4];
        let pressure = vec![1., 2., 3., 4.];

        let mut stroke = Stroke::new(x, y, timestamp, pressure);
        let meta_stroke = stroke.clone();
        stroke.meta.insert(String::from("someVal"), json!(7.1));
        stroke.meta.insert(String::from("someArr"), json!([1, 2, 3, 4]));
        stroke.meta.insert(String::from("someVec"), json!(vec![1., 2., 3., 4., 5., 32.]));
        stroke.meta.insert(String::from("someObj"), json!(meta_stroke));


        let strokes = vec![stroke.clone(), stroke.clone()];
        let sketch = Sketch::new(strokes);

        json_serializer::dumps_stroke(&stroke);
        json_serializer::dumps_sketch(&sketch);

        let val_some_val = stroke.meta.get("someVal").unwrap().clone();
        let val_some_arr = stroke.meta.get("someArr").unwrap().clone();
        let val_some_vec = stroke.meta.get("someVec").unwrap().clone();
        let val_some_obj = stroke.meta.get("someObj").unwrap().clone();

        let some_val: f64 = serde_json::from_value(val_some_val).unwrap();
        let some_arr: Vec<i32> = serde_json::from_value(val_some_arr).unwrap();
        let some_vec: Vec<f64> = serde_json::from_value(val_some_vec).unwrap();
        let some_obj: Stroke = serde_json::from_value(val_some_obj).unwrap();

        assert_eq!(some_val, 7.1);
        assert_eq!(some_arr, [1, 2, 3, 4]);
        assert_eq!(some_vec, vec![1., 2., 3., 4., 5., 32.]);
        assert_eq!(some_obj, meta_stroke);
    }

    #[test]
    fn equals() {
        let x0 = vec![1., 2., 3., 4., 5., 32.];
        let y0 = vec![1., 2., 3., 4., 5.];
        let timestamp0 = vec![1, 2, 3, 4];
        let pressure0 = vec![1., 2., 3., 4.];

        let x1 = vec![1., 2., 3., 4., 5., 32.];
        let y1 = vec![1., 2., 3., 4., 5.];
        let timestamp1 = vec![1, 2, 3, 4];
        let pressure1 = vec![1., 2., 3., 4.];

        let x2 = vec![2.];
        let y2 = vec![2.];
        let timestamp2 = vec![2];
        let pressure2 = vec![2.];

        let stroke0 = Stroke::new(x0, y0, timestamp0, pressure0);
        let stroke1 = Stroke::new(x1, y1, timestamp1, pressure1);
        let stroke2 = Stroke::new(x2, y2, timestamp2, pressure2);

        assert_eq!(stroke0, stroke1);
        assert_eq!(stroke1, stroke0);
        assert_ne!(stroke0, stroke2);
        assert_ne!(stroke2, stroke0);
        assert_ne!(stroke1, stroke2);
        assert_ne!(stroke2, stroke1);
    }

    #[test]
    fn serialization() {
        let x = vec![1., 2., 3., 4., 5., 32.];
        let y = vec![1., 2., 3., 4., 5.];
        let timestamp = vec![1, 2, 3, 4];
        let pressure = vec![1., 2., 3., 4.];

        let stroke = Stroke::new(x.clone(), y.clone(), timestamp.clone(), pressure.clone());

        let strokes = vec![stroke.clone(), stroke.clone()];
        let sketch = Sketch::new(strokes);

        let json_stroke = json_serializer::dumps_stroke(&stroke);
        let json_sketch = json_serializer::dumps_sketch(&sketch);

        let deserialized_stroke = json_serializer::loads_stroke(json_stroke).unwrap();
        let deserialized_sketch = json_serializer::loads_sketch(json_sketch).unwrap();

        assert_eq!(deserialized_stroke.x, x);
        assert_eq!(deserialized_stroke.y, y);
        assert_eq!(deserialized_stroke.timestamp, timestamp);
        assert_eq!(deserialized_stroke.pressure, pressure);

        assert_eq!(stroke, deserialized_stroke);
        assert_eq!(sketch, deserialized_sketch);
    }

    #[test]
    fn file_writing() {
        let x = vec![1., 2., 3., 4., 5., 32.];
        let y = vec![1., 2., 3., 4., 5.];
        let timestamp = vec![1, 2, 3, 4];
        let pressure = vec![1., 2., 3., 4.];

        let stroke = Stroke::new(x.clone(), y.clone(), timestamp.clone(), pressure.clone());
        let strokes = vec![stroke.clone(), stroke.clone()];
        let sketch = Sketch::new(strokes);

        let dir = tempdir().unwrap();
        let file_path_stroke = String::from(dir.path().join("temp_stroke.json").to_str().unwrap());
        let file_path_sketch = String::from(dir.path().join("temp_sketch.json").to_str().unwrap());

        json_serializer::dump_stroke(&stroke, &file_path_stroke);
        json_serializer::dump_sketch(&sketch, &file_path_sketch);

        let loaded_stroke = json_serializer::load_stroke(&file_path_stroke).unwrap();
        let loaded_sketch = json_serializer::load_sketch(&file_path_sketch).unwrap();

        assert_eq!(stroke, loaded_stroke);
        assert_eq!(sketch, loaded_sketch);

        let result = dir.close();
        match result {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn stroke_offset() {
        let x = vec![0., 0., 1., 1.];
        let y = vec![0., 1., 1., 0.];

        let mut stroke = Stroke::new(x, y, vec![], vec![]);
        stroke.offset(Some(4.2), Some(-1.));

        assert_eq!(stroke.x, [4.2, 4.2, 5.2, 5.2]);
        assert_eq!(stroke.y, [-1., 0., 0., -1.]);

        assert_eq!(stroke.x_min(), 4.2);
        assert_eq!(stroke.x_max(), 5.2);
        assert_eq!(stroke.y_min(), -1.);
        assert_eq!(stroke.y_max(), 0.);
    }

    #[test]
    fn sketch_offset() {
        let x = vec![0., 0., 1., 1.];
        let y = vec![0., 1., 1., 0.];

        let stroke_one = Stroke::new(x.to_vec(), y.to_vec(), vec![], vec![]);
        let stroke_two = Stroke::new(x, y, vec![], vec![]);
        let strokes = vec![stroke_one, stroke_two];

        let mut sketch = Sketch::new(strokes);
        sketch.offset(Some(4.2), Some(-1.));

        assert_eq!(sketch.strokes[0].x, [4.2, 4.2, 5.2, 5.2]);
        assert_eq!(sketch.strokes[0].y, [-1., 0., 0., -1.]);
        assert_eq!(sketch.strokes[1].x, [4.2, 4.2, 5.2, 5.2]);
        assert_eq!(sketch.strokes[1].y, [-1., 0., 0., -1.]);
    }

    #[test]
    fn stroke_scale() {
        let x = vec![0., 0., 1., 1.];
        let y = vec![0., 1., 1., 0.];
        let x_scaled = vec![0., 0., 4.2, 4.2];
        let y_scaled = vec![0., -1., -1., 0.];

        let mut stroke = Stroke::new(x, y, vec![], vec![]);
        stroke.scale(Some(4.2), Some(-1.));

        assert_eq!(stroke.x, x_scaled);
        assert_eq!(stroke.y, y_scaled);

        assert_eq!(stroke.x_min(), 0.);
        assert_eq!(stroke.x_max(), 4.2);
        assert_eq!(stroke.y_min(), -1.);
        assert_eq!(stroke.y_max(), 0.);
    }

    #[test]
    fn sketch_scale() {
        let x = vec![0., 0., 1., 1.];
        let y = vec![0., 1., 1., 0.];
        let x_scaled = vec![0., 0., 4.2, 4.2];
        let y_scaled = vec![0., -1., -1., 0.];

        let stroke_one = Stroke::new(x.to_vec(), y.to_vec(), vec![], vec![]);
        let stroke_two = Stroke::new(x, y, vec![], vec![]);
        let strokes = vec![stroke_one, stroke_two];

        let mut sketch = Sketch::new(strokes);
        sketch.scale(Some(4.2), Some(-1.));

        assert_eq!(sketch.strokes[0].x, x_scaled);
        assert_eq!(sketch.strokes[0].y, y_scaled);
        assert_eq!(sketch.strokes[1].x, x_scaled);
        assert_eq!(sketch.strokes[1].y, y_scaled);
    }

    fn generate_sketch() -> Sketch {
        let x1 = vec![1., 2.];
        let y1 = vec![0.9, 0.3];
        let x2 = vec![1.2, 1.5];
        let y2 = vec![0.6, 0.7];

        let s1 = Stroke::new(x1, y1, vec![], vec![]);
        let s2 = Stroke::new(x2, y2, vec![], vec![]);
        let strokes = vec![s1, s2];

        return Sketch::new(strokes);
    }

    #[test]
    fn sketch_normalize_default() {
        let mut sketch = generate_sketch();
        sketch.normalize(1., false);

        assert_eq!(sketch.strokes[0].x[0], 0. as f64);
        assert_eq!(sketch.strokes[0].x[1], 1. as f64);
        assert_eq!(sketch.strokes[0].y[1], 0. as f64);
        assert_eq!(sketch.strokes[0].y[0], 1. as f64);
        for i in 0..sketch.strokes[1].len() {
            let x = sketch.strokes[1].x[i];
            let y = sketch.strokes[1].y[i];
            assert!(x > 0.);
            assert!(x < 1.);
            assert!(y > 0.);
            assert!(y < 1.);
        }

        sketch.normalize(2., false);
        assert_eq!(sketch.strokes[0].x[1], 2. as f64);
        assert_eq!(sketch.strokes[0].y[0], 2. as f64);
    }

    #[test]
    fn sketch_normalize_aspect_ratio_preserving() {
        let mut sketch = generate_sketch();
        sketch.normalize(1., true);

        assert_eq!(sketch.strokes[0].x[1], 1.);
        assert!(sketch.strokes[0].y[0] < 1.);

        sketch = generate_sketch();
        sketch.strokes[1].y[1] = 20.;
        sketch.normalize(1., true);
        assert!(sketch.strokes[0].x[1] < 1.);
        assert!(sketch.strokes[0].y[0] < 1.);
        assert_eq!(sketch.strokes[1].y[1], 1.);
    }

    #[test]
    fn sketch_processing() {
        let x1 = vec![1., 2., 3., 4.];
        let y1 = vec![1., 2., 3., 4.];
        let t1 = vec![1, 2, 3, 4];
        let p1 = vec![1., 1., 1., 1.];
        let s1 = Stroke::new(x1, y1, t1, p1);

        let x2 = vec![2., 3., 3., 5.];
        let y2 = vec![5., 3., 3., 1.];
        let t2 = vec![1, 2, 3, 4];
        let p2 = vec![1., 1., 1., 1.];
        let s2 = Stroke::new(x2, y2, t2, p2);

        let x3 = vec![3., 3.];
        let y3 = vec![3., 3.];
        let t3 = vec![1, 2];
        let p3 = vec![1., 1.];
        let s3 = Stroke::new(x3, y3, t3, p3);

        let strokes = vec![s1, s2, s3];
        let mut sketch = Sketch::new(strokes);

        sketch.remove_duplicate_dots();
        assert_eq!(sketch.strokes[0].len(), 4);
        assert_eq!(sketch.strokes[1].len(), 3);

        sketch.remove_single_dot_strokes();
        assert_eq!(sketch.len(), 2);
    }
}
