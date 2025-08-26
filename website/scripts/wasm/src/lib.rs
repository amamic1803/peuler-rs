use js_sys::{Array, Number, Object, Reflect};
use peuler::{PEuler as libPEuler, ProjectEuler};
use pmath::statistics::Sample as libSample;
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

    pub fn benchmark(&self, id: usize) -> Result<Object, JsValue> {
        let (res, dur) = self
            .inner
            .benchmark(id)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let obj = Object::new();
        Reflect::set(&obj, &JsValue::from_str("result"), &JsValue::from_str(&res))?;
        Reflect::set(
            &obj,
            &JsValue::from_str("duration"),
            &Number::from(dur.as_nanos() as f64),
        )?;

        Ok(obj)
    }
}
impl Default for PEuler {
    fn default() -> Self {
        Self::new()
    }
}

#[wasm_bindgen]
pub struct Sample {
    inner: libSample<f64>,
}
#[wasm_bindgen]
impl Sample {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: libSample::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }

    pub fn push(&mut self, value: f64) {
        self.inner.push(value);
    }

    pub fn mean(&self) -> Option<f64> {
        self.inner.mean()
    }

    pub fn stddev(&self) -> Option<f64> {
        self.inner.sample_stddev()
    }
}
impl Default for Sample {
    fn default() -> Self {
        Self::new()
    }
}
