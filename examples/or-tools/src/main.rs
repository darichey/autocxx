// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

fn main() {
include_cpp! {
    #include "ortools/base/commandlineflags.h"
    #include "ortools/base/integral_types.h"
    #include "ortools/base/logging.h"
    #include "ortools/base/map_util.h"
    #include "ortools/constraint_solver/constraint_solveri.h"
    safety!(unsafe_ffi)
    generate!("operations_research::Solver") // using example of nqueens.cpp
}

use ffi::ToCppString;

fn main() {
    let _ = ffi::operations_research::Solver::make_unique("hello".to_cpp());
}
