// Original publisher here: https://exercism.io/tracks/rust/exercises/react/solutions/1e89202826fc49bc9d7b634b6a4daf67
use std::collections::HashMap;

pub type InputCellID = usize;
pub type ComputeCellID = usize;
pub type CallbackID = usize;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

#[derive(PartialEq, Debug)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

type ComputeFn<'a, T> = (CellID, Vec<CellID>, Box<dyn Fn(&[T]) -> T + 'a>);
type CallBackFn<'a, T> = (ComputeCellID, CallbackID, Box<dyn FnMut(T) -> () + 'a>);

#[derive(Default)]
pub struct Reactor<'a, T> {
    input: HashMap<InputCellID, CellID>,
    compute: HashMap<CellID, ComputeCellID>,
    state: HashMap<CellID, T>,
    compute_fn: Vec<ComputeFn<'a, T>>,
    cb: Vec<CallBackFn<'a, T>>,
    seq: usize,
}

impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {
            input: HashMap::new(),
            compute: HashMap::new(),
            state: HashMap::new(),
            compute_fn: Vec::new(),
            cb: Vec::new(),
            seq: 0,
        }
    }

    pub fn create_input(&mut self, initial: T) -> InputCellID {
        let input_id = self.seq;
        self.seq += 1;
        let cell_id = CellID::Input(input_id);
        self.input.insert(input_id, cell_id);
        self.state.insert(cell_id, initial);
        input_id
    }

    pub fn create_compute<F>(
        &mut self,
        dependencies: &[CellID],
        compute_func: F,
    ) -> Result<ComputeCellID, CellID>
    where
        F: Fn(&[T]) -> T + 'a,
    {
        let compute_id = self.seq;
        self.seq += 1;
        let cell_id = CellID::Compute(compute_id);

        if let Some(non_exist_id) = dependencies.iter().find(|id| self.state.get(id).is_none()) {
            return Err(*non_exist_id);
        }

        let value = self.calculate(dependencies, &compute_func);
        self.compute_fn
            .push((cell_id, dependencies.to_vec(), Box::new(compute_func)));
        self.state.insert(cell_id, value);
        self.compute.insert(cell_id, compute_id);
        Ok(compute_id)
    }

    fn calculate(&self, dependencies: &[CellID], compute_func: &dyn Fn(&[T]) -> T) -> T {
        let values = dependencies
            .iter()
            .map(|id| {
                let value = self.state.get(id);
                if value.is_none() {
                    unimplemented!("Should never happen!")
                } else {
                    *value.unwrap()
                }
            })
            .collect::<Vec<_>>();
        compute_func(values.as_slice())
    }

    pub fn value(&self, id: CellID) -> Option<T> {
        self.state.get(&id).cloned()
    }

    pub fn set_value(&mut self, id: InputCellID, new_value: T) -> bool {
        if let Some(cellid) = self.input.get(&id) {
            if let Some(value) = self.state.get_mut(cellid) {
                if *value != new_value {
                    *value = new_value;
                    self.update();
                }
            }
            true
        } else {
            false
        }
    }

    fn update(&mut self) {
        for (id, dependencies, compute_func) in self.compute_fn.iter() {
            let new_value = self.calculate(dependencies, &**compute_func);
            if let Some(value) = self.state.get_mut(id) {
                if *value != new_value {
                    *value = new_value;
                    if let Some(compute_cell) = self.compute.get(id) {
                        let filtered = self
                            .cb
                            .iter_mut()
                            .filter(|(cell_id, _, _)| cell_id == compute_cell);
                        for (_, _, cb) in filtered {
                            cb(new_value);
                        }
                    }
                }
            }
        }
    }

    pub fn add_callback<F: FnMut(T) -> () + 'a>(
        &mut self,
        id: ComputeCellID,
        callback: F,
    ) -> Option<CallbackID> {
        if self.compute.iter().any(|(&_, &v)| v == id) {
            let callbackid = self.seq;
            self.seq += 1;
            self.cb.push((id, callbackid, Box::new(callback)));
            Some(callbackid)
        } else {
            None
        }
    }

    pub fn remove_callback(
        &mut self,
        cell: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        if self
            .compute
            .iter()
            .any(|(_, compute_id)| *compute_id == cell)
        {
            if let Some(pos) = self
                .cb
                .iter()
                .position(|(compute_id, cb_id, _)| *compute_id == cell && *cb_id == callback)
            {
                self.cb.remove(pos);
                Ok(())
            } else {
                Err(RemoveCallbackError::NonexistentCallback)
            }
        } else {
            Err(RemoveCallbackError::NonexistentCell)
        }
    }
}
