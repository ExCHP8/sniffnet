use crate::enums::language::Language;
use iced::widget::Text;

pub fn choose_adapters_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Select network adapter to inspect",
        Language::IT => "Seleziona la scheda di rete da ispezionare",
        Language::FR => "Sélectionnez une carte réseau à inspecter",
        Language::ES => "Seleccione el adaptador de red que desea inspeccionar",
        Language::PL => "Wybierz adapter sieciowy do inspekcji",
        Language::CN => "选择需要监控的网络适配器",
    })
}

pub fn application_protocol_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Application protocol",
        Language::IT => "Protocollo applicativo",
        Language::FR => "Protocole applicatif",
        Language::ES => "Protocolo de aplicación",
        Language::PL => "Protokół aplikacji",
        Language::CN => "目标应用层协议",
    }
}

pub fn select_filters_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Select filters to be applied on network traffic",
        Language::IT => "Seleziona i filtri da applicare al traffico di rete",
        Language::FR => "Sélectionnez les filtres à appliquer sur le traffic réseau",
        Language::ES => "Seleccionar los filtros que se aplicarán al tráfico de red",
        Language::PL => "Wybierz filtry, które mają być zastosowane na ruchu sieciowym",
        Language::CN => "选择需要监控的目标",
    })
}

pub fn start_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Start!",
        Language::IT => "Avvia!",
        Language::FR => "Commencer!",
        Language::ES => "¡Empieza!",
        Language::PL => "Rozpocznij!",
        Language::CN => "开始!",
    }
}

pub fn address_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "\nAddress:",
        Language::IT => "\nIndirizzo:",
        Language::FR => "\nAdresse:",
        Language::ES => "\nDirección:",
        Language::PL => "\nAdres:",
        Language::CN => "\n网络地址:",
    }
}

pub fn addresses_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "\nAddresses:",
        Language::IT => "\nIndirizzi:",
        Language::FR => "\nAdresses:",
        Language::ES => "\nDirecciones:",
        Language::PL => "\nAdresy:",
        Language::CN => "\n网络地址",
    }
}

pub fn ip_version_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "IP version",
        Language::IT => "Versione IP",
        Language::FR => "Version IP",
        Language::ES => "Versión IP",
        Language::PL => "Wersja IP",
        Language::CN => "目标IP协议版本",
    })
}

pub fn transport_protocol_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Transport protocol",
        Language::IT => "Protocollo di trasporto",
        Language::FR => "Protocole de transport",
        Language::ES => "Protocolo de transporte",
        Language::PL => "Protokół transportowy",
        Language::CN => "目标传输协议",
    }
}

pub fn traffic_rate_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Traffic rate:",
        Language::IT => "Intensità del traffico:",
        Language::FR => "Fréquence du traffic:",
        Language::ES => "Tasa de tráfico:",
        Language::PL => "Prędkość ruchu:",
        Language::CN => "网络速率图",
    })
}

pub fn relevant_connections_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Relevant connections:",
        Language::IT => "Connessioni rilevanti:",
        Language::FR => "Connexions pertinentes:",
        Language::ES => "Conexiones Relevantes:",
        Language::PL => "Istotne połączenia:",
        Language::CN => "连接详情:",
    })
}

pub fn settings_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Settings",
        Language::IT => "Impostazioni",
        Language::FR => "Paramètres",
        Language::ES => "Ajustes",
        Language::PL => "Ustawienia",
        Language::CN => "设置",
    }
}

pub fn yes_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Yes",
        Language::IT => "Sì",
        Language::FR => "Oui",
        Language::ES => "Sí",
        Language::PL => "Tak",
        Language::CN => "是",
    })
}

pub fn ask_quit_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Are you sure you want to quit this analysis?",
        Language::IT => "Sei sicuro di voler interrompere questa analisi?",
        Language::FR => "Êtes-vous sûr de vouloir quitter l'application ?",
        Language::ES => "¿Estás seguro de que quieres dejar este análisis?",
        Language::PL => "Czy na pewno chcesz zakończyć analizę?",
        Language::CN => "您确定退出当前监控吗?",
    })
}

pub fn quit_analysis_translation(language: Language) -> String {
    match language {
        Language::EN => "Quit analysis".to_string(),
        Language::IT => "Interrompi analisi".to_string(),
        Language::FR => "Quitter l'analyse".to_string(),
        Language::ES => "Quitar el análisis".to_string(),
        Language::PL => "Zakończ analize".to_string(),
        Language::CN => "退出监控".to_string(),
    }
}

pub fn ask_clear_all_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Are you sure you want to clear notifications?",
        Language::IT => "Sei sicuro di voler eliminare le notifiche?",
        Language::FR => "Êtes-vous sûr de vouloir effacer les notifications ?",
        Language::ES => "¿Seguro que quieres borrar las notificaciones?",
        Language::PL => "Czy na pewno chcesz wyczyścić powiadomienia?",
        Language::CN => "确定清除所有通知?",
    })
}

pub fn clear_all_translation(language: Language) -> String {
    match language {
        Language::EN => "Clear all".to_string(),
        Language::IT => "Elimina tutte".to_string(),
        Language::FR => "Tout effacer".to_string(),
        Language::ES => "Borrar todo".to_string(),
        Language::PL => "Wyczyść wszystko".to_string(),
        Language::CN => "清除所有".to_string(),
    }
}

pub fn hide_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Hide",
        Language::IT => "Nascondi",
        Language::FR => "Masquer",
        Language::ES => "Ocultar",
        Language::PL => "Ukryj",
        Language::CN => "隐藏",
    }
}

pub fn no_addresses_translation(language: Language, adapter: &str) -> Text<'static> {
    Text::new(match language {
        Language::EN => format!("No traffic can be observed because the adapter you selected has no active addresses...\n\n\
                                 Network adapter: {adapter}\n\n\
                                 If you are sure you are connected to the internet, try choosing a different adapter."),
        Language::IT => format!("Non è osservabile alcun traffico perché l'adattatore di rete selezionato non ha indirizzi attivi...\n\n\
                                Adattatore di rete: {adapter}\n\n\
                                Se sei sicuro di essere connesso ad internet, prova a scegliere un adattatore diverso."),
        Language::FR => format!("Aucun trafic ne peut être observé, car la carte réseau que vous avez saisie n'a pas d'adresse...\n\n\
                                Carte réseau : {adapter}\n\n\
                                Si vous êtes sûr d'être connecté à internet, essayez une autre carte."),
        Language::ES => format!("No se puede observar ningún tráfico porque el adaptador seleccionado no tiene direcciones activas...\n\n\
                                 Adaptador de red : {adapter}\n\n\
                                 Si estás seguro de que estás conectado a Internet, prueba a elegir otro adaptador."),
        Language::PL => format!("Nie można zaobserwować żadnego ruchu, ponieważ wybrany adapter nie ma aktywnych adresów...\n\n\
                                 Adapter sieciowy: {adapter}\n\n\
                                 Jeśli jesteś pewien, że jesteś podłączony do internetu, spróbuj wybrać inny adapter."),
        Language::CN => format!("您选择的网络适配器当前无活动网络......\n\n\
                                网络适配器: {adapter}\n\n\
                                如果您确信您已成功连接互联网, 请尝试选择其他网络适配器."),
    })
}

pub fn waiting_translation(language: Language, adapter: &str) -> Text<'static> {
    Text::new(match language {
        Language::EN => format!("No traffic has been observed yet. Waiting for network packets...\n\n\
                                 Network adapter: {adapter}\n\n\
                                 Are you sure you are connected to the internet and you have selected the correct adapter?"),
        Language::IT => format!("Nessun tipo di traffico è stato osservato finora. Attendo pacchetti di rete...\n\n\
                                Adattatore di rete: {adapter}\n\n\
                                Sei sicuro di esser connesso ad internet e di aver selezionato l'adattatore corretto?"),
        Language::FR => format!("Aucun trafic n'a été capturé pour le moment. En attente de paquets...\n\n\
                                Carte réseau : {adapter}\n\n\
                                Êtes-vous sûr d'être connecté à internet et d'avoir selectionné la bonne carte réseau ?"),
        Language::ES => format!("Aún no se ha captado tráfico. Esperando paquetes...\n\n\
                                 Adaptador de red : {adapter}\n\n\
                                 ¿Está seguro de que está conectado a Internet y ha seleccionado la tarjeta de red correcta?"),
        Language::PL => format!("Nie zaobserowano żadnego ruchu sieciowego. Oczekiwanie na pakiety...\n\n\
                                 Adapter sieciowy: {adapter}\n\n\
                                 Czy na pewno jesteś podłączony do internetu i wybrałeś właściwy adapter?"),
        Language::CN => format!("暂无流量数据. 等待网络活动中......\n\n\
                                 网络适配器: {adapter}\n\n\
                                 您确信您已成功连接到互联网, 并选择了当前正在使用的的网络适配器吗?"),
    })
}

pub fn some_observed_translation(
    language: Language,
    observed: &str,
    filters: &str,
) -> Text<'static> {
    Text::new(match language {
        Language::EN => format!("Total intercepted packets: {observed}\n\n\
                                 Filtered packets: 0\n\n\
                                 Some packets have been intercepted, but still none has been selected according to the filters you specified...\n\n{filters}"),
        Language::IT => format!("Totale pacchetti intercettati: {observed}\n\n\
                                 Pacchetti filtrati: 0\n\n\
                                 Alcuni pacchetti sono stati intercettati, ma ancora nessuno è stato selezionato secondo i filtri specificati...\n\n{filters}"),
        Language::FR => format!("Total des paquets interceptés: {observed}\n\n\
                                 Paquets filtrés: 0\n\n\
                                 Certains paquets ont été interceptés, mais aucun ne satisfait les critères des filtres sélectionnés...\n\n{filters}"),
        Language::ES => format!("Total de paquetes interceptados: {observed}\n\n\
                                 Paquetes filtrados: 0\n\n\
                                 Se interceptaron algunos paquetes, pero ninguno de ellos cumplía los criterios de los filtros seleccionados...\n\n{filters}"),
        Language::PL => format!("Suma przechwyconych pakietów: {observed}\n\n\
                                 Przefiltrowane pakiety: 0\n\n\
                                 Niektóre pakiety zostały przechwycone, ale żaden nie został wybrany zgodnie z wskazanymi filtrami...\n\n{filters}"),
        Language::CN => format!("监测到的数据包总数: {observed}\n\n\
                                 目标数据包总数: 0\n\n\
                                 当前已监测到一些数据包, 但其中并未包含您的目标数据包......\n\n{filters}"),
    })
}

pub fn filtered_packets_translation(
    language: Language,
    filtered: &str,
    percentage: &str,
) -> Text<'static> {
    Text::new(match language {
        Language::EN => format!("Filtered packets:\n   {filtered} ({percentage} of the total)"),
        Language::IT => format!("Pacchetti filtrati:\n   {filtered} ({percentage} del totale)"),
        Language::FR => format!("Paquets filtrés:\n   {filtered} ({percentage} du total)"),
        Language::ES => format!("Paquetes filtrados:\n   {filtered} ({percentage} del total)"),
        Language::PL => format!("Przefiltrowane pakiety:\n   {filtered} ({percentage} z całości)"),
        Language::CN => {
            format!("目标数据包计数:\n   {filtered} (占所有数据包的 {percentage})")
        }
    })
}

pub fn filtered_bytes_translation(
    language: Language,
    filtered: &str,
    percentage: &str,
) -> Text<'static> {
    Text::new(match language {
        Language::EN => format!("Filtered bytes:\n   {filtered} ({percentage} of the total)"),
        Language::IT => format!("Byte filtrati:\n   {filtered} ({percentage} del totale)"),
        Language::FR => format!("Octets filtrés:\n   {filtered} ({percentage} du total)"),
        Language::ES => format!("Bytes filtrados:\n   {filtered} ({percentage} del total)"),
        Language::PL => format!("Przechwycone bajty:\n   {filtered} ({percentage} całości)"),
        Language::CN => {
            format!("目标网络流量计数:\n   {filtered} (占所有网络流量的 {percentage})")
        }
    })
}

pub fn filtered_application_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Filtered packets per application protocol:",
        Language::IT => "Pacchetti filtrati per protocollo applicativo:",
        Language::FR => "Paquets filtrés par protocole applicatif:",
        Language::ES => "Paquetes filtrados por protocolo de aplicación:",
        Language::PL => "Przefiltrowane pakiety według protokołu aplikacji:",
        Language::CN => "按应用层协议分类的目标数据包计数:",
    })
}

pub fn no_favorites_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Nothing to show at the moment.\n\
                         To add a connection to your favorites, click on the star symbol near the connection.",
        Language::IT => "Nulla da vedere per il momento.\n\
                         Per aggiungere una connessione ai tuoi preferiti, clicca sul simbolo della stella vicino alla connessione.",
        Language::FR => "Rien a voir pour le moment.\n\
                         Pour ajouter une connexion à vos favoris, cliquez sur l'étoile à côté de la connexion.",
        Language::ES => "Nada que mostrar por el momento.\n\
                         Para añadir una conexión a sus favoritos, haga clic en el símbolo de la estrella situado junto a la conexión.",
        Language::PL => "Nie ma nic do pokazania w tej chwili.\n\
                         Aby dodać połączenie do ulubionych, kliknij na ikonę 'gwiazdki' obok połączenia.",
        Language::CN => "收藏夹还是空的.\n\
                         小贴士: 点击连接右侧的小星星即可收藏到这里哦.",
    })
}

pub fn error_translation(language: Language, error: &str) -> Text<'static> {
    Text::new(match language {
        Language::EN => format!(
            "An error occurred! \n\n\
                                {error}"
        ),
        Language::IT => format!(
            "Si è verificato un errore! \n\n\
                                {error}"
        ),
        Language::FR => format!(
            "Une erreur est survenue! \n\n\
                                {error}"
        ),
        Language::ES => format!(
            "¡Se ha producido un error! \n\n\
                                {error}"
        ),
        Language::PL => format!(
            "Wystąpił błąd! \n\n\
                                {error}"
        ),
        Language::CN => format!(
            "发生了一些错误! \n\n\
                                {error}"
        ),
    })
}

pub fn both_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "both",
        Language::IT => "entrambi",
        Language::FR => "les deux",
        Language::ES => "ambos",
        Language::PL => "oba",
        Language::CN => "皆需",
    }
}

// pub fn all_protocols_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "All protocols",
//         Language::IT => "Tutti i protocolli",
//         Language::FR => "Tous les protocoles",
//         Language::ES => "Todos los protocolos",
//         Language::PL => "Wszystkie protokoły",
//     }
// }

pub fn all_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "All",
        Language::IT => "Tutti",
        Language::FR => "Tous",
        Language::ES => "Todos",
        Language::PL => "Wszystkie",
        Language::CN => "所有",
    }
}

pub fn packets_chart_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "packets per second",
        Language::IT => "pacchetti al secondo",
        Language::FR => "paquets par seconde",
        Language::ES => "paquetes por segundo",
        Language::PL => "pakiety na sekundę",
        Language::CN => "数据包",
    }
}

pub fn bytes_chart_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "bytes per second",
        Language::IT => "byte al secondo",
        Language::FR => "octets par seconde",
        Language::ES => "bytes por segundo",
        Language::PL => "bajty na sekundę",
        Language::CN => "网络流量",
    }
}

pub fn recent_report_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "most recent",
        Language::IT => "più recenti",
        Language::FR => "la plus récente",
        Language::ES => "más reciente",
        Language::PL => "najnowsze",
        Language::CN => "按时间",
    }
}

pub fn packets_report_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "most packets",
        Language::IT => "più pacchetti",
        Language::FR => "le plus de paquets",
        Language::ES => "mayoría de los paquetes",
        Language::PL => "najwięcej pakietów",
        Language::CN => "按数据包",
    }
}

pub fn bytes_report_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "most bytes",
        Language::IT => "più byte",
        Language::FR => "le plus de données",
        Language::ES => "mayoría de los bytes",
        Language::PL => "najwięcej bajtów",
        Language::CN => "按流量",
    }
}

pub fn favorite_report_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "favorites",
        Language::IT => "preferiti",
        Language::FR => "favoris",
        Language::ES => "favoritos",
        Language::PL => "ulubione",
        Language::CN => "收藏夹",
    }
}

pub fn notifications_title_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Customize your notifications",
        Language::IT => "Personalizza le tue notifiche",
        Language::FR => "Personnalisez vos notifications",
        Language::ES => "Personaliza tus notificaciones",
        Language::PL => "Dostosuj powiadomienia",
        Language::CN => "自定义通知",
    })
}

pub fn appearance_title_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Choose your favorite theme",
        Language::IT => "Scegli il tuo tema preferito",
        Language::FR => "Sélectionnez votre thème préféré",
        Language::ES => "Elige tu tema favorito",
        Language::PL => "Wybierz swój ulubiony motyw",
        Language::CN => "选择您喜欢的主题",
    })
}

pub fn languages_title_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "Select your language",
        Language::IT => "Seleziona la lingua",
        Language::FR => "Sélectionnez votre langue",
        Language::ES => "Selecciona tu idioma",
        Language::PL => "Wybierz język",
        Language::CN => "选择显示语言",
    })
}

pub fn active_filters_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Active filters:",
        Language::IT => "Filtri attivi:",
        Language::FR => "Filtres actifs",
        Language::ES => "Filtros activos:",
        Language::PL => "Aktywne filtry:",
        Language::CN => "活动的过滤器:",
    }
}

pub fn none_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "none",
        Language::IT => "nessuno",
        Language::FR => "aucun",
        Language::ES => "ninguno",
        Language::PL => "brak",
        Language::CN => "无",
    }
}

pub fn yeti_night_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Sniffnet's original dark theme",
        Language::IT => "Il tema scuro originale di Sniffnet",
        Language::FR => "Thème original sombre de Sniffnet",
        Language::ES => "Tema oscuro original de Sniffnet",
        Language::PL => "Oryginalny, ciemny motyw Sniffnet",
        Language::CN => "Sniffnet暗黑",
    }
}

pub fn yeti_day_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Sniffnet's original light theme",
        Language::IT => "Il tema chiaro originale di Sniffnet",
        Language::FR => "Thème original clair de Sniffnet",
        Language::ES => "Tema claro original de Sniffnet",
        Language::PL => "Oryginalny, jasny motyw Sniffnet",
        Language::CN => "Sniffnet浅色",
    }
}

pub fn deep_sea_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "To dive into network traffic",
        Language::IT => "Per immergersi nel traffico di rete",
        Language::FR => "Pour plonger dans votre trafic réseau",
        Language::ES => "Para sumergirse en el tráfico de la red",
        Language::PL => "Aby zanurzyć się w ruchu sieciowym",
        Language::CN => "潜入网络活动的海洋",
    }
}

pub fn mon_amour_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Lovely theme made for dreamers",
        Language::IT => "Tema incantevole fatto per i sognatori",
        Language::FR => "Thème romantique fait pour les rêveurs",
        Language::ES => "Tema encantador hecho para soñadores",
        Language::PL => "Uroczy motyw stworzony dla marzycieli",
        Language::CN => "梦想家的主题",
    }
}

pub fn incoming_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Incoming",
        Language::IT => "In entrata",
        Language::FR => "Entrant",
        Language::ES => "Entrante",
        Language::PL => "Przychodzące",
        Language::CN => "入站",
    }
}

pub fn outgoing_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Outgoing",
        Language::IT => "In uscita",
        Language::FR => "Sortant",
        Language::ES => "Saliente",
        Language::PL => "Wychodzące",
        Language::CN => "出站",
    }
}

pub fn notifications_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::FR => "Notifications",
        Language::IT => "Notifiche",
        Language::ES => "Notificaciones",
        Language::PL => "Powiadomienia",
        Language::CN => "通知",
    }
}

pub fn style_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::FR => "Style",
        Language::IT => "Stile",
        Language::ES => "Estilo",
        Language::PL => "Styl",
        Language::CN => "主题",
    }
}

pub fn language_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Language",
        Language::IT => "Lingua",
        Language::FR => "Langue",
        Language::ES => "Idioma",
        Language::PL => "Język",
        Language::CN => "语言",
    }
}

pub fn overview_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Overview",
        Language::IT => "Panoramica",
        Language::FR => "Résumé",
        Language::ES => "Resumen",
        Language::PL => "Przegląd",
        Language::CN => "概览",
    }
}

// pub fn inspect_translation(language: Language) -> &'static str {
//     match language {
//         Language::EN => "Inspect",
//         Language::IT => "Ispeziona",
//         Language::FR => "Inspecter",
//         Language::ES => "Inspeccionar",
//         Language::PL => "Sprawdź",
//     }
// }

pub fn packets_threshold_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Notify me when a packets threshold is exceeded",
        Language::IT => "Notificami quando una soglia di pacchetti è superata",
        Language::FR => "Me notifier lorsqu'un seuil de paquet est atteint",
        Language::ES => "Notificarme cuando se supere un límite de paquetes",
        Language::PL => "Powiadom mnie, gdy zostanie przekroczony próg pakietów",
        Language::CN => "超过设定的数据包数量阈值时通知我",
    }
}

pub fn bytes_threshold_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Notify me when a bytes threshold is exceeded",
        Language::IT => "Notificami quando una soglia di byte è superata",
        Language::FR => "Me notifier lorsqu'un seuil de donnée est atteint",
        Language::ES => "Notificarme cuando se exceda un límite de bytes",
        Language::PL => "Powiadom mnie, gdy zostanie przekroczony próg bajtów",
        Language::CN => "超过设定的网络流量阈值时通知我",
    }
}

pub fn per_second_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "(per second)",
        Language::IT => "(al secondo)",
        Language::FR => "(par seconde)",
        Language::ES => "(por segundo)",
        Language::PL => "(na sekundę)",
        Language::CN => "(每秒) ",
    }
}

pub fn specify_multiples_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "; you can also specify 'K', 'M' and 'G'",
        Language::IT => "; puoi anche specificare 'K', 'M' e 'G'",
        Language::FR => "; vous pouvez également spécifier 'K', 'M' et 'G'",
        Language::ES => "; también puede especificar 'K', 'M' y 'G'",
        Language::PL => "; możesz również określić 'K', 'M' i 'G'",
        Language::CN => "您可指定 'K', 'M', 'G'",
    }
}

pub fn favorite_notification_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Notify me when new data are exchanged from my favorites",
        Language::IT => "Notificami quando nuovi dati sono scambiati dai miei preferiti",
        Language::FR => "Notifiez-moi lorsque des données sont échangées depuis mes favoris",
        Language::ES => "Notificarme cuando se intercambien nuevos datos de mis favoritos",
        Language::PL => "Powiadom mnie, gdy nowe dane z moich ulubionych zostaną wymienione",
        Language::CN => "收藏夹内的连接有新活动时通知我",
    }
}

pub fn threshold_translation(language: Language) -> String {
    match language {
        Language::EN => "Threshold: ".to_string(),
        Language::IT => "Soglia: ".to_string(),
        Language::FR => "Seuil: ".to_string(),
        Language::ES => "Límite: ".to_string(),
        Language::PL => "Próg: ".to_string(),
        Language::CN => "阈值: ".to_string(),
    }
}

pub fn volume_translation(language: Language, value: u8) -> String {
    match language {
        Language::EN | Language::IT | Language::FR => format!("Volume: {value:^3}%"),
        Language::ES => format!("Volumen: {value:^3}%"),
        Language::PL => format!("Głośność: {value:^3}%"),
        Language::CN => format!("通知音量: {value:^3}%"),
    }
}

pub fn sound_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Sound:",
        Language::IT => "Suono:",
        Language::FR => "Son:",
        Language::ES => "Sonido:",
        Language::PL => "Dźwięk:",
        Language::CN => "通知音:",
    }
}

pub fn open_report_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Open full report",
        Language::IT => "Apri report completo",
        Language::FR => "Ouvrir le rapport complet",
        Language::ES => "Abrir el informe completo",
        Language::PL => "Otwórz pełny raport",
        Language::CN => "打开完整报告",
    }
}

pub fn bytes_exceeded_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Bytes threshold exceeded!",
        Language::IT => "Soglia di Byte superata!",
        Language::FR => "Seuil de donnée atteint!",
        Language::ES => "¡Límite de bytes superado!",
        Language::PL => "Próg bajtów przekroczony!",
        Language::CN => "达到设定的网络流量阈值!",
    }
}

pub fn bytes_exceeded_value_translation(language: Language, value: &str) -> String {
    let trimmed_value = value.trim();
    match language {
        Language::EN => format!("{trimmed_value} bytes have been exchanged"),
        Language::IT => format!("{trimmed_value} byte sono stati scambiati"),
        Language::FR => format!("{trimmed_value} octets ont été échangé"),
        Language::ES => format!("{trimmed_value} byte/s han sido intercambiado/s"),
        Language::PL => format!("Wymieniono {trimmed_value} bajtów"),
        Language::CN => format!("已交换字节 {trimmed_value}"),
    }
}

pub fn packets_exceeded_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Packets threshold exceeded!",
        Language::IT => "Soglia di pacchetti superata!",
        Language::FR => "Le seuil de paquet a été atteint!",
        Language::ES => "¡Se ha superado el límite de paquetes!",
        Language::PL => "Próg pakietów przekroczony!",
        Language::CN => "达到设定的数据包数量阈值!",
    }
}

pub fn packets_exceeded_value_translation(language: Language, value: u32) -> String {
    match language {
        Language::EN => format!("{value} packets have been exchanged"),
        Language::IT => format!("{value} pacchetti sono stati scambiati"),
        Language::FR => match value {
            1 => "1 paquet a été échangé".to_owned(),
            npackets => format!("{npackets} paquets ont été échangés"),
        },
        Language::ES => format!("{value} paquete/s han sido intercambiado/s"),
        Language::PL => format!("Wymieniono {value} pakietów"),
        Language::CN => format!("已交换数据包 {value}"),
    }
}

pub fn favorite_transmitted_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "New data exchanged from favorites!",
        Language::IT => "Nuovi dati scambiati dai preferiti!",
        Language::FR => "Nouvel échange de donnée depuis un favori!",
        Language::ES => "¡Nuevos datos intercambiados de favoritos!",
        Language::PL => "Nowe dane wymienione z ulubionych!",
        Language::CN => "收藏夹内的连接有新活动!",
    }
}

pub fn no_notifications_set_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => "You haven't enabled notifications yet!\n\n\
                                 After you will enable them, this page will display a log of your notifications\n\n\
                                 You can enable notifications from settings:",
        Language::IT => "Non hai ancora abilitato le notifiche!\n\n\
                                Dopo che le avrai abilitate, questa pagina mostrerà una collezione delle tue notifiche\n\n\
                                Puoi abilitare le notifiche dalle impostazioni:",
        Language::FR => "Vous n'avez pas activé les notifications!\n\n\
                                    Une fois activées, cette page affichera le journal des notifications\n\n\
                                    Vous pouvez les activer dans les paramètres:",
        Language::ES => "¡Aún no has activado las notificaciones!\n\n\
                                 Después de activarlas, esta página mostrará un registro de sus notificaciones\n\n\
                                 Puedes activar las notificaciones desde los ajustes:",
        Language::PL => "Nie włączyłeś jeszcze powiadomień!\n\n\
                                 Po ich włączeniu, ta strona wyświetli dziennik twoich powiadomień\n\n\
                                 Możesz włączyć powiadomienia w ustawieniach:",
        Language::CN => "您还没有设定任何通知!\n\n\
                                 启用它们后，此页面将显示您的通知日志\n\n\
                                 您可以从设置中设定:",
    })
}

pub fn no_notifications_received_translation(language: Language) -> Text<'static> {
    Text::new(match language {
        Language::EN => {
            "Nothing to see at the moment...\n\n\
                                 When you will receive a notification, it will be displayed here"
        }
        Language::IT => {
            "Nulla da vedere al momento...\n\n\
                                Quando riceverai una notifica, essa verrà mostrata qui"
        }
        Language::FR => {
            "Rien à voir pour le moment...\n\n\
                                 Lorsque vous recevrez une notification, elle s'affichera ici"
        }
        Language::ES => {
            "Nada que ver por el momento...\n\n\
                                 Cuando reciba una notificación, aparecerá aquí"
        }
        Language::PL => {
            "Nic do wyświetlenia w tej chwili...\n\n\
                                 Gdy otrzymasz powiadomienie, pojawi się ono tutaj"
        }
        Language::CN => {
            "还没有任何通知...\n\n\
                                 当您收到通知时，它会显示在这里"
        }
    })
}

pub fn only_last_30_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Only the last 30 notifications are displayed",
        Language::IT => "Solo le ultime 30 notifiche sono mostrate",
        Language::FR => "Seulement les 30 dernières notifications sont affichées",
        Language::ES => "Sólo se muestran las últimas 30 notificaciones",
        Language::PL => "Wyświetlane jest tylko 30 ostatnich powiadomień",
        Language::CN => "仅显示最近 30 条通知",
    }
}
