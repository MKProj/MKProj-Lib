#[allow(unused_variables)]
pub mod phaktionz {
    //! Phaktionz CLI is goaled to bring information on Phaktionz directly to your system

    pub fn update() {
        //! Updates Phaktionz CLI
        //!
        //! The Update functions essentially just cargo installs Phaktionz
        //!
        //! ```rust
        //! let update = std::process::Command::new("cargo")
        //!     .arg("install")
        //!     .arg("phaktionz")
        //!     .spawn();
        //! ```
        let update = std::process::Command::new("cargo")
            .arg("install")
            .arg("phaktionz")
            .spawn();
    }
    pub fn fetch(sub_cmd: String, format: String) {
        //! Fetches different aspects in HTML, EPUB or PDF
        let _url: String = format!(
            "https://github.com/MKProj/Phaktionz/raw/main/DOCS/{}.{}",
            sub_cmd, format
        );
        let _wget = std::process::Command::new("wget")
            .arg(&_url)
            .spawn()
            .expect("failed to execute process");
    }
    pub mod rules {
        //! The Rules Module is based on describing various aspects of the game
        /// This is used to describe the types of Cards (Summons, Invocations)
        /// ```
        /// pub struct Card{
        ///     pub name: String, // This gives the type of Card it is ex. Striker
        ///     pub description: String // This gives a description of what type does
        /// }
        /// ```
        pub struct Card {
            pub name: String,
            pub description: String,
        }

        /// This is used to describe the types in Summons and Invocations  
        /// - Summon Types:
        ///     - Striker
        ///     - Tech  
        /// - Invocation Types:
        ///     - Regular
        ///     - Counter
        ///     - Weapon
        ///     - Realm  
        ///
        /// If a new card type would be made, then a change could look like this:  
        ///
        /// Before
        /// `pub fn types(summmons: [Card; 2], invocations: [Card; 4])`  
        ///
        /// After (example new summon type)
        /// `pub fn types(summmons: [Card; 3], invocations: [Card; 4])`
        pub fn types(summmons: [Card; 2], invocations: [Card; 4]) {
            println!("Summmons: ");
            // This will cycle through all the struct Card fields
            for i in 0..summmons.len() {
                println!("\t{}: {}", summmons[i].name, summmons[i].description);
            }
            println!("Invocations");
            for i in 0..invocations.len() {
                println!("\t{}: {}", invocations[i].name, invocations[i].description);
            }
        }
        ///This is used to describe the game mechanics of Phaktionz
        /// ```rust
        ///     pub fn game() {
        ///         let mech: [String; 5] = [ // This is used to store all the game mech information
        ///         String::from("??? When a Summon battles it becomes disabled (turned sideways)"),
        ///         String::from("??? To place a Tier 2 or higher summon, you must demote Tiers total to the Summon???s Tier.\n\t??? For example, a Tier 2 may be placed by demoting a Tier 2 or 2 Tier 1s."),
        ///         String::from("??? At the start of the game, after the turn order is chosen, both players may mulligan any cards in their hand once."),
        ///         String::from("??? If a card???s ability were to break one of these rules, the card???s ability takes precedence."),
        ///         String::from( "??? When battling, a Player takes DMG equal to the difference between the Summons.\n\t??? If a Summon that battles has less DMG than the opposing, no DMG is dealt.\n\t??? If a Summon that battles has more DMG than the opposing, the Opponent takes the difference, and the Summon is demoted, except if it???s Tier 3+.")
        ///     ];
        ///         for i in 0..mech.len() {
        ///             println!("\t{}", mech[i]);
        ///         }
        /// }
        /// ```
        pub fn game() {
            let mech: [String; 5] = [
            String::from("??? When a Summon battles it becomes disabled (turned sideways)"),
            String::from("??? To place a Tier 2 or higher summon, you must demote Tiers total to the Summon???s Tier.\n\t??? For example, a Tier 2 may be placed by demoting a Tier 2 or 2 Tier 1s."),
            String::from("??? At the start of the game, after the turn order is chosen, both players may mulligan any cards in their hand once."),
            String::from("??? If a card???s ability were to break one of these rules, the card???s ability takes precedence."),
            String::from( "??? When battling, a Player takes DMG equal to the difference between the Summons.\n\t??? If a Summon that battles has less DMG than the opposing, no DMG is dealt.\n\t??? If a Summon that battles has more DMG than the opposing, the Opponent takes the difference, and the Summon is demoted, except if it???s Tier 3+.")
        ];
            for i in 0..mech.len() {
                println!("\t{}", mech[i]);
            }
        }
        /// This describes the various terms in Phaktionz
        ///```rust
        ///pub fn terms() {
        ///let keywords:[String; 11] = [ //Contains all keywords which is iterated through
        ///    String::from("Summons: Units that battle in the battlefield"),
        ///    String::from("Invocations: Sorcery that may be cast to gain benefit"),
        ///    String::from("Abled: The position in which a unit may battle"),
        ///    String::from("Disabled: The position in which a unit is unable to battle (This is done with your Summon being sideways)"),
        ///    String::from("Demote: To have a summon leave the battlefield"),
        ///    String::from("Exile: To remove from play a summon"),
        ///    String::from("Tiers: Represents the rank of a summon, Tier 1 being the lowest and 3 the highest"),
        ///    String::from("DMG: The amount of cards a summon can deal an opponent to lose"),
        ///    String::from("Fizzle: To stop an opponent's play"),
        ///    String::from("L/x: Limit x per turn"),
        ///    String::from("Lx: Limit x per match")
        ///];
        ///     for i in 0..keywords.len() {
        ///         println!("\t{}", keywords[i]);
        ///     }  
        ///}
        ///
        /// ```
        pub fn terms() {
            let keywords:[String; 11] = [
            String::from("Summons: Units that battle in the battlefield"),
            String::from("Invocations: Sorcery that may be cast to gain benefit"),
            String::from("Abled: The position in which a unit may battle"),
            String::from("Disabled: The position in which a unit is unable to battle (This is done with your Summon being sideways)"),
            String::from("Demote: To have a summon leave the battlefield"),
            String::from("Exile: To remove from play a summon"),
            String::from("Tiers: Represents the rank of a summon, Tier 1 being the lowest and 3 the highest"),
            String::from("DMG: The amount of cards a summon can deal an opponent to lose"),
            String::from("Fizzle: To stop an opponent's play"),
            String::from("L/x: Limit x per turn"),
            String::from("Lx: Limit x per match")
        ];
            for i in 0..keywords.len() {
                println!("\t{}", keywords[i]);
            }
        }
        /// This describe the Creation Pile
        /// ```rust
        ///pub fn c_pile() {
        ///   let cp:[String; 4] = [
        ///    String::from("??? A CP Card may be added to your hand if it satisfies the Card???s Create condition."),
        ///    String::from("??? Limit: 10 (In Format V1 & V2)"),
        ///    String::from("??? Abilities that include ???CP??? refer to Creation Pile"),
        ///    String::from("??? CP Cards are identified with CP in the top left along where Tier or Invocation type is located.")
        ///];
        ///     for i in 0..cp.len() {
        ///         println!("\t{}", cp[i]);
        ///     }
        ///}
        /// ```
        pub fn c_pile() {
            let cp:[String; 4] = [
            String::from("??? A CP Card may be added to your hand if it satisfies the Card???s Create condition."),
            String::from("??? Limit: 10 (In Format V1 & V2)"),
            String::from("??? Abilities that include ???CP??? refer to Creation Pile"),
            String::from("??? CP Cards are identified with CP in the top left along where Tier or Invocation type is located.")
        ];
            for i in 0..cp.len() {
                println!("\t{}", cp[i]);
            }
        }
        /// This describes the Promote Mechanic
        pub fn promote() {
            let pr = String::from(
                "
    Promote brings forth two new Tiers, 0 and 4. A Tier 0/4 Summon has a limit of 1.\n
        Tier 0:\n
            \tTier 0???s are Summons that are placed and have a realm invocation ability,\n
            \tas well as having these attributes:\n
            \t??? Tier : 0\n
            \t??? DMG: 0\n
            \t??? Type: T/S\n
            \t??? Promote: Yes\n
            \t??? All Tier 0???s may not be battled, and treated as a Realm Invocation\n
        Promote: \n
        \tTo Promote a Tier 0, is to flip it to it???s other side where it resides\n
        \tas Tier 4, and is placed at the Tier 3 location. To Promote, the player must\n
        \tsatisfy a Promote condition that is described on the Card. If you control a\n
        \tSummons on the Battlefield and choose to Promote, the Summons will be demoted.\n
        Tier 4:\n
        \tTier 4???s are usually a win condition card and are built to not stay on the\n
        \tfield for long. While you control a Tier 4 you cannot have any other Summons\n
        \ton the Battlefield. As well as that, they have the following attributes:\n
            \t??? Tier: 4\n
            \t??? DMG: 7/8\n
            \t??? Type: T/S\n
            \t??? All Tier 4???s cannot be demoted in Battle nor be demoted by any Abilities\n
            \t??? At the end of each End Phase the player takes Damage equal to it???s DMG\n
                \t\t??? Refusal to pay will result in it being exiled.
        ",
            );
            println!("{}", pr);
        }
        ///This is used to list all the rule commands (all available functions)
        pub fn list() {
            let opt: [String; 6] = [
                String::from("types"),
                String::from("game-mech"),
                String::from("terms"),
                String::from("c-pile"),
                String::from("promote"),
                String::from("all"),
            ];
            let desc: [String; 6] = [
                String::from("Lists out all Summon/Invocation Types"),
                String::from("Lists out all the Game Mechanics"),
                String::from("Lists out all the key terms"),
                String::from("Describes what Creation Pile is"),
                String::from("Describes what Promote is"),
                String::from("Does all commands above"),
            ];
            println!("Available Options for Rules: ");
            for i in 0..opt.len() {
                println!("\t{}: {}", opt[i], desc[i]);
            }
        }
        /// This is used to process all the rule commands
        pub fn rules(option: String, summmons: [Card; 2], invocations: [Card; 4]) {
            let opt: [String; 7] = [
                String::from("types"),
                String::from("game-mech"),
                String::from("terms"),
                String::from("c-pile"),
                String::from("promote"),
                String::from("all"),
                String::from("list"),
            ];

            if option == opt[0] {
                types(summmons, invocations);
            } else if option == opt[1] {
                game();
            } else if option == opt[2] {
                terms();
            } else if option == opt[3] {
                c_pile();
            } else if option == opt[4] {
                promote();
            } else if option == opt[5] {
                types(summmons, invocations);
                println!("\n");
                game();
                println!("\n");
                terms();
                println!("\n");
                c_pile();
            } else if option == opt[6] {
                list();
            }
        }
    }

    /*pub mod story {
        //! This is used to describe the various episode concepts
        /// To give info on how to access each episode concept
        /// ```rust
        ///pub struct Episode {
        ///     pub name: String,
        ///     pub season: i32,
        ///     pub episode: i32,
        ///     pub url: String,
        ///  }
        /// ```
        pub struct Episode {
            pub name: String,
            pub season: i32,
            pub episode: i32,
            pub url: String,
        }
        /// This is used to access the pdf files in a temporary access way
        ///
        /// You will need the url that is provided by the Episode struct, and pdf application that is
        /// required by the story subcommand.
        ///
        /// ```rust
        /// pub fn read(url: String, pdf_application: String) {
        ///     let read = std::process::Command::new(pdf_application).arg(url).output();
        ///     // This issues a command line command to use the pdf application and load the pdf with url
        /// }
        /// ```
        pub fn read(url: String, pdf_application: String) {
            let read = std::process::Command::new(pdf_application)
                .arg(url)
                .output();
        }
    }*/

    pub mod book {
        //! This is used to initialize and serve the phaktionz markdown book
        pub fn init() {
            let gitclone = std::process::Command::new("git")
                .arg("clone")
                .arg("https://github.com/MKProj/Phaktionz-Book.git")
                .spawn();
        }
        pub fn serve() {
            println!("Make sure you have first ran phaktionz init!");
            let serve = std::process::Command::new("mdbook")
                .arg("serve")
                .arg("Phaktionz-Book")
                .spawn();
        }
    }

    pub mod info {
        //! This is used to give information about the Factions Categories
        pub struct Category {
            pub name: String,
            pub desc: String,
        }

        pub fn Info(name: String, cat: Vec<Category>) {
            let mut i = 0;
            while i < cat.len() {
                if name == cat[i].name {
                    println!("{}", cat[i].desc);
                } else if name == "list" {
                    println!("\t*{}", cat[i].name);
                }
                i += 1;
            }
        }
    }

    pub mod profiles {
        //! This is used to make the character profiles
        pub fn prof(option: String, characters: Vec<Profile>) {
            for i in 0..characters.len() {
                if option == characters[i].fname {
                    println!(
                        "Name: {} {}\nAge: {}\nHeight: {}\nFaction: {}\nDecks: {}\nDescription: \n{}\n",
                        characters[i].fname,
                        characters[i].lname,
                        characters[i].age,
                        characters[i].height,
                        characters[i].factions,
                        characters[i].decks,
                        characters[i].description
                    );
                } else if option == "list" {
                    println!("\t*{}", characters[i].fname);
                }
            }
        }
        pub struct Profile {
            pub fname: String,
            pub lname: String,
            pub age: i64,
            pub height: String,
            pub factions: String,
            pub decks: String,
            pub description: String,
        }
    }
}
