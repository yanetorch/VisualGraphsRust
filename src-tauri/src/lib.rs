/// Содержит прикладные функции
mod tasks {
    pub mod task0;
}

/// Хранит функции для дерганья с фронта.
mod client_functions {}

/// Хранит модели. Оч неоднозначная штука
mod model {
    pub mod error;
    pub mod graph;
    pub mod vertex;
}

/// Какие-то фундаментальные алгоритмы для графов
mod algorithm {
    mod dijkstra;
    mod kruskal_mst;
    mod prufer_code;
}

pub fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
