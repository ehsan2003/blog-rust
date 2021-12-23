#[macro_export]
macro_rules! make_interactor_setup {
        ($iname:ident,[$(($name:ident , $value:expr , $type:ty)),+]) => {
            struct CreationResult {
                interactor: $iname,
                $(
                    $name: std::sync::Arc<$type>,
                )+
            }
            fn create_interactor() -> CreationResult {
                $(
                    let $name = std::sync::Arc::new($value);
                )+
                let interactor = $iname::new($(
                        $name.clone(),
                    )+);
                CreationResult {
                    interactor,
                    $(
                        $name,
                    )+
                }
            }
        };
    }
