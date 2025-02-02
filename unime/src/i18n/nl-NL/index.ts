import type { Translation } from '../i18n-types';

const nl = {
  ONBOARDING: {
    WELCOME: {
      GREETING: 'Welkom bij',
      WHAT_IS_UNIME_1: 'UniMe verbindt je digitale wereld, veilig en beschermd.',
      WHAT_IS_UNIME_2: 'Maak een gloednieuw profiel aan om te beginnen.',
      CREATE_NEW_PROFILE: 'Maak nieuw profiel aan',
      SELECT_LANGUAGE: 'Selecteer taal',
    },
    PLEDGE: {
      NAVBAR_TITLE: 'UniMe Belofte',
      TITLE_1: 'Geen rare',
      TITLE_2: 'toestanden',
      SUBTITLE: 'Hier is onze belofte aan jou',
      ITEM_1: {
        TITLE: 'Wij delen jouw gegevens niet',
        DESCRIPTION:
          'Jouw gegevens zijn van jou en alleen jij bepaalt met wie je ze deelt. Punt. Sterker nog, jouw gegevens komen nooit in aanraking met een van onze systemen — tenzij je kiest voor een van de cloudopslag opties.',
      },
      ITEM_2: {
        TITLE: 'We zullen geen trackers toevoegen',
        DESCRIPTION:
          'We volgen je acties niet achter de schermen. Punt. Niet om te testen of om welke reden dan ook. Dat beloven we. We verzamelen ook geen anonieme apparaatinformatie or gebruiksstatistieken. Deze beslissing maakt de ontwikkeling van de app wat moeilijker voor ons, maar we geloven dat het de juiste beslissing is.',
      },
      ITEM_3: {
        TITLE: 'Je bezit jouw informatie',
        DESCRIPTION: 'Wij geloven dat het tijd wordt dat jij weer de baas wordt van je eigen persoonlijke informatie.',
      },
    },
    TERMS: {
      NAVBAR_TITLE: 'Algemene Voorwaarden',
      TITLE_1: 'Hier zijn de minder interessante',
      TITLE_2: 'dingen',
      SUBTITLE: 'Ja, we weten het. Wij raden het toch aan deze informatie aandachtig te lezen.',
      OWNERSHIP: {
        TITLE: 'Eigendom',
        DESCRIPTION: 'Ik begrijp dat ik volledig verantwoordelijk ben voor mijn gegevens',
      },
    },
    CUSTOMIZE: {
      NAVBAR_TITLE: 'Personalisatie',
      NAME: {
        TITLE_1: 'Laten we beginnen! Kies een',
        TITLE_2: 'profielnaam',
        SUBTITLE: 'Jouw profielinformatie verlaat nooit jouw apparaat',
        INPUT_PLACEHOLDER: 'Profielnaam invoeren',
      },
      THEME: {
        TITLE_1: 'Kies het uiterlijk',
        TITLE_2: 'van je app',
        SUBTITLE: 'Ben jij meer een nachtuil?',
      },
      PICTURE: {
        TITLE_1: 'Stel een',
        TITLE_2: 'profielfoto in',
        SUBTITLE: 'Maak het je eigen.',
      },
      SKIP: {
        TITLE: 'Sla personalisatie over',
        TEXT: 'Weet je het zeker? Je kunt het uiterlijk van de app later aanpassen in de instellingen.',
        CONFIRM: 'Ja',
        ABORT: 'Nee, ga door',
      },
    },
    PASSWORD: {
      NAVBAR_TITLE: 'Wachtwoord',
      TITLE_1: 'Stel je nieuwe',
      TITLE_2: 'wachtwoord in',
      SUBTITLE: 'Je moet een sterk wachtwoord kiezen om je gegevens veilig te versleutelen.',
      INPUT_PLACEHOLDER: 'Voer wachtwoord in',
      CONFIRM: {
        NAVBAR_TITLE: 'Wachtwoord Bevestigen',
        TITLE_1: 'Bevestig je nieuwe',
        TITLE_2: 'wachtwoord',
        SUBTITLE: 'Je moet je wachtwoord bevestigen om er zeker van te zijn dat je het correct hebt getypt.',
        INPUT_PLACEHOLDER: 'Typ je wachtwoord opnieuw in',
        MATCH: 'Wachtwoorden komen overeen',
        NO_MATCH: 'Wachtwoorden komen niet overeen',
      },
      COMPLETED: {
        NAVBAR_TITLE: 'Wachtwoord Instellen',
        TITLE_1: 'Je UniMe profiel is nu',
        TITLE_2: 'beschermd',
        MESSAGE_1: 'Veilig & Beveiligd',
        MESSAGE_2: 'Goed Gedaan',
      },
    },
  },
  SETTINGS: {
    NAVBAR_TITLE: 'Instellingen',
    PROFILE: {
      TITLE: 'Mijn profiel',
      PROFILE_NAME: {
        TITLE: 'Profielnaam',
        NAVBAR_TITLE: 'Bewerk profielnaam',
        INPUT_PLACEHOLDER: 'Profielnaam invoeren',
        CONFIRM: 'Wijzigingen opslaan',
      },
      DISPLAY_PICTURE: {
        TITLE: 'Profielfoto',
        EDIT: 'Bewerk',
        CHANGE: 'Selecteer een nieuwe afbeelding',
      },
      DELETE_PROFILE: {
        TITLE: 'Verwijder profiel',
      },
    },
    APP: {
      TITLE: 'App instellingen',
      NAVBAR_TITLE: 'App Instellingen',
      LANGUAGE: {
        TITLE: 'Taal',
        NAVBAR_TITLE: 'Selecteer Taal',
        COMING_SOON: 'Binnenkort beschikbaar',
      },
      THEME: {
        TITLE: 'Thema',
        NAVBAR_TITLE: 'Selecteer Thema',
      },
      PASSWORD: {
        TITLE: 'Wachtwoord',
      },
      ONBOARDING_JOURNEY: {
        TITLE: 'Onboarding journey',
        BUTTON_TEXT: 'Herstart',
      },
      HINTS_AND_TIPS: {
        TITLE: 'Hints en tips',
        BUTTON_TEXT: 'Opnieuw instellen',
      },
      DEVELOPER_MODE: {
        TITLE: 'Ontwikkelaarsmodus',
      },
    },
    BACKUP_RECOVERY: {
      TITLE: 'Back-up en herstel',
    },
    LOG_OUT: {
      TITLE: 'Log uit',
    },
    THEME: {
      SYSTEM: 'Systeem',
      LIGHT: 'Licht',
      DARK: 'Donker',
    },
    PASSWORD: {
      POLICY: {
        TITLE: 'Wachtwoord',
        UPPERCASE_LETTER: 'Hoofdletters',
        LOWERCASE_LETTER: 'Kleine letters',
        NUMBER: 'Nummers',
        CHARACTERS: 'Karakters',
      },
    },
    RESET_APP: {
      TITLE: 'App opnieuw instellen',
      DESCRIPTION: 'Weet je zeker dat je de app opnieuw wilt instellen en alle gegevens wilt verwijderen?',
      CONFIRM: 'Ja, verwijder alles',
      CANCEL: 'Nee, bewaar mijn profiel',
    },
    ACCOUNT: 'Profiel',
    SUPPORT: {
      TITLE: 'Support',
      ABOUT: {
        TITLE: 'Over UniMe',
        NAVBAR_TITLE: 'Over UniMe',
        BUILT_WITH: 'Ontwikkeld met Tauri',
      },
      FEEDBACK: {
        TITLE: 'Stuur feedback',
      },
    },
  },
  LOCK_SCREEN: {
    PASSWORD_INPUT_PLACEHOLDER: 'Voer je wachtwoord in',
    BUTTON_TEXT: 'Ontgrendel wallet',
    FORGOT_PASSWORD: 'Wachtwoord vergeten?',
  },
  ME: {
    BOTTOM_NAVIGATION_TITLE: 'Me',
    GREETINGS: {
      GREETING_0: 'Hey',
      GREETING_1: 'Hallo',
      GREETING_2: 'Welkom terug',
      GREETING_3: 'Hoi',
      GREETING_4: 'Dag',
    },
    DEMO: {
      TEXT_1: 'Bezoek',
      TEXT_2: 'op een desktopcomputer om te beginnen.',
    },
    CREDENTIAL_TABS: {
      ALL: 'Alle',
      DATA: 'Data',
      BADGES: 'Badges',
    },
    EMPTY_CREDENTIALS: {
      TITLE: 'Momenteel is het hier wat rustig',
      SUBTITLE: 'Wat denk je ervan om nieuwe credentials toe te voegen om je digitale "me" te starten?',
    },
    FAVORITES: 'Mijn favorieten',
  },
  ACTIVITY: {
    BOTTOM_NAVIGATION_TITLE: 'Activiteiten',
    NAVBAR_TITLE: 'Verbonden',
    TABS: {
      CONNECTIONS: 'Connecties',
      TIMELINE: 'Tijdlijn',
    },
  },
  SCAN: {
    BOTTOM_NAVIGATION_TITLE: 'Scan',
    TITLE_1: 'Scan een',
    TITLE_2: 'QR Code',
    SUBTITLE: 'Breng een QR-code in beeld op dit scherm om een interactie te starten.',
    NO_PERMISSION_1: 'Geen recht om toegang',
    NO_PERMISSION_2: 'te krijgen tot de camera',
    CREDENTIAL_OFFER: {
      NAVBAR_TITLE: 'Credential Aanbod',
      DESCRIPTION: 'biedt u de volgende credentials aan',
      ACCEPT: 'Accepteer credentials',
    },
    CONNECTION_REQUEST: {
      NAVBAR_TITLE: 'Credential Aanvraag',
      TITLE: 'Nieuwe connectie',
      DESCRIPTION: 'Accepteer alleen nieuwe connecties die je herkent en vertrouwt',
      CONNECTED_PREVIOUSLY: 'Eerder verbonden',
      ACCEPT: 'Accepteer connectie',
    },
    SHARE_CREDENTIALS: {
      NAVBAR_TITLE: 'Gegevens Delen',
      DESCRIPTION: 'vraagt de volgende credentials op',
      REQUESTED: 'Aangevraagd',
      APPROVE: 'Accepteer verzoek',
    },
  },
  CONNECTION: {
    TABS: {
      SUMMARY: 'Overzicht',
      DATA: 'Data',
      ACTIVITY: 'Activiteit',
    },
    SUMMARY: {
      EMPTY: 'Nog geen connecties.',
      TITLE: 'Verbonden met',
      FIRST_CONNECTED: 'Eerst gebruikt',
      LAST_CONNECTED: 'Laatst gebruikt',
    },
    DATA: {
      EMPTY: 'Nog geen data.',
    },
  },
  TIMELINE: {
    EMPTY: 'Nog geen activiteit.',
  },
  SEARCH: {
    INPUT_PLACEHOLDER: 'Zoeken',
    NO_QUERY: {
      TITLE: 'Geef een zoekopdracht op.',
      DESCRIPTION: 'Zoek hier naar je credentials en badges.',
    },
    NO_RESULTS: {
      TITLE: 'Geen resultaten gevonden',
      DESCRIPTION: 'Probeer iets anders te zoeken.',
    },
  },
  CREDENTIAL: {
    NAVBAR_TITLE: 'Credential Informatie',
  },
  BADGE: {
    NAVBAR_TITLE: 'Badge Informatie',
    DETAILS: {
      VALID: 'Geldig',
      ISSUED_BY: 'Uitgegeven door',
      DESCRIPTION: 'Beschrijving',
      CONTENTS: 'Inhoud',
    },
  },
  CANCEL: 'Annuleren',
  CLOSE: 'Sluiten',
  CONTINUE: 'Doorgaan',
  SKIP: 'Overslaan',
  REJECT: 'Weigeren',
} satisfies Translation;

export default nl;
