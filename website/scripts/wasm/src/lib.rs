use js_sys::{Array, Number, Object, Reflect};
use peuler::{PEuler as libPEuler, ProjectEuler};
use pmath::statistics::Sample;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct PEuler {
    inner: libPEuler,
}
#[wasm_bindgen]
impl PEuler {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: libPEuler::new(),
        }
    }

    pub fn problems(&self) -> Result<Array, JsValue> {
        let arr = Array::new();
        for p in self.inner.problems() {
            let obj = Object::new();
            Reflect::set(&obj, &JsValue::from_str("id"), &Number::from(p.id() as u32))?;
            Reflect::set(
                &obj,
                &JsValue::from_str("title"),
                &JsValue::from_str(p.title()),
            )?;
            arr.push(&obj);
        }
        Ok(arr)
    }

    pub fn solve(&self, id: usize) -> Result<String, JsValue> {
        self.inner
            .solve(id)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    pub fn benchmark(&self, id: usize, iterations: usize) -> Result<Object, JsValue> {
        if iterations < 3 {
            return Err(JsValue::from_str(
                "There must be at least 3 iterations for benchmarking.",
            ));
        }

        let mut result = None;
        let mut sample = Sample::new();
        for _ in 0..iterations {
            match self.inner.benchmark(id) {
                Ok((res, dur)) => {
                    match &result {
                        Some(prev_res) => {
                            if prev_res != &res {
                                return Err(JsValue::from_str(
                                    "Inconsistent results across iterations.",
                                ));
                            }
                        }
                        None => {
                            result = Some(res);
                        }
                    }
                    sample.push(dur.as_nanos());
                }
                Err(e) => return Err(JsValue::from_str(&e.to_string())),
            }
        }

        let obj = Object::new();
        Reflect::set(
            &obj,
            &JsValue::from_str("result"),
            &JsValue::from_str(&result.unwrap_throw()),
        )?;
        Reflect::set(
            &obj,
            &JsValue::from_str("iterations"),
            &Number::from(iterations as u32),
        )?;
        Reflect::set(
            &obj,
            &JsValue::from_str("mean"),
            &Number::from(sample.mean().unwrap_throw()),
        )?;
        Reflect::set(
            &obj,
            &JsValue::from_str("stddev"),
            &Number::from(sample.sample_stddev().unwrap_throw()),
        )?;

        Ok(obj)
    }
}
impl Default for PEuler {
    fn default() -> Self {
        Self::new()
    }
}
