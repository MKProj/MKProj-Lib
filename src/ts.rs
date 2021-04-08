pub mod tomorrow_study {
    //! To bring Tomorrow Study's material directly to your system
    pub struct Content {
        pub name: String,
        pub author: String,
        pub id: String,
        pub url: String,
    }

    pub struct Category {
        pub cont: [Content; 2],
    }

    pub fn List(cont_type: Category) {
        let mut i = 0;
        println!("| Name | Author | ID |");
        while i < cont_type.cont.len() {
            println!(
                "| {} | {} | {} |",
                cont_type.cont[i].name, cont_type.cont[i].author, cont_type.cont[i].id
            );
            i += 1;
        }
    }
}
