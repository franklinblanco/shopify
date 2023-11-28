
/// Enum representing country codes.
#[derive(Debug, Serialize, Deserialize)]
pub enum CountryCode {
    /// Ascension Island.
    AC,
    /// Andorra.
    AD,
    /// United Arab Emirates.
    AE,
    /// Afghanistan.
    AF,
    /// Antigua & Barbuda.
    AG,
    /// Anguilla.
    AI,
    /// Albania.
    AL,
    /// Armenia.
    AM,
    /// Netherlands Antilles.
    AN,
    /// Angola.
    AO,
    /// Argentina.
    AR,
    /// Austria.
    AT,
    /// Australia.
    AU,
    /// Aruba.
    AW,
    /// Åland Islands.
    AX,
    /// Azerbaijan.
    AZ,
    /// Bosnia & Herzegovina.
    BA,
    /// Barbados.
    BB,
    /// Bangladesh.
    BD,
    /// Belgium.
    BE,
    /// Burkina Faso.
    BF,
    /// Bulgaria.
    BG,
    /// Bahrain.
    BH,
    /// Burundi.
    BI,
    /// Benin.
    BJ,
    /// St. Barthélemy.
    BL,
    /// Bermuda.
    BM,
    /// Brunei.
    BN,
    /// Bolivia.
    BO,
    /// Caribbean Netherlands.
    BQ,
    /// Brazil.
    BR,
    /// Bahamas.
    BS,
    /// Bhutan.
    BT,
    /// Bouvet Island.
    BV,
    /// Botswana.
    BW,
    /// Belarus.
    BY,
    /// Belize.
    BZ,
    /// Canada.
    CA,
    /// Cocos (Keeling) Islands.
    CC,
    /// Congo - Kinshasa.
    CD,
    /// Central African Republic.
    CF,
    /// Congo - Brazzaville.
    CG,
    /// Switzerland.
    CH,
    /// Côte d’Ivoire.
    CI,
    /// Cook Islands.
    CK,
    /// Chile.
    CL,
    /// Cameroon.
    CM,
    /// China.
    CN,
    /// Colombia.
    CO,
    /// Costa Rica.
    CR,
    /// Cuba.
    CU,
    /// Cape Verde.
    CV,
    /// Curaçao.
    CW,
    /// Christmas Island.
    CX,
    /// Cyprus.
    CY,
    /// Czechia.
    CZ,
    /// Germany.
    DE,
    /// Djibouti.
    DJ,
    /// Denmark.
    DK,
    /// Dominica.
    DM,
    /// Dominican Republic.
    DO,
    /// Algeria.
    DZ,
    /// Ecuador.
    EC,
    /// Estonia.
    EE,
    /// Egypt.
    EG,
    /// Western Sahara.
    EH,
    /// Eritrea.
    ER,
    /// Spain.
    ES,
    /// Ethiopia.
    ET,
    /// Finland.
    FI,
    /// Fiji.
    FJ,
    /// Falkland Islands.
    FK,
    /// Faroe Islands.
    FO,
    /// France.
    FR,
    /// Gabon.
    GA,
    /// United Kingdom.
    GB,
    /// Grenada.
    GD,
    /// Georgia.
    GE,
    /// French Guiana.
    GF,
    /// Guernsey.
    GG,
    /// Ghana.
    GH,
    /// Gibraltar.
    GI,
    /// Greenland.
    GL,
    /// Gambia.
    GM,
    /// Guinea.
    GN,
    /// Guadeloupe.
    GP,
    /// Equatorial Guinea.
    GQ,
    /// Greece.
    GR,
    /// South Georgia & South Sandwich Islands.
    GS,
    /// Guatemala.
    GT,
    /// Guinea-Bissau.
    GW,
    /// Guyana.
    GY,
    /// Hong Kong SAR.
    HK,
    /// Heard & McDonald Islands.
    HM,
    /// Honduras.
    HN,
    /// Croatia.
    HR,
    /// Haiti.
    HT,
    /// Hungary.
    HU,
    /// Indonesia.
    ID,
    /// Ireland.
    IE,
    /// Israel.
    IL,
    /// Isle of Man.
    IM,
    /// India.
    IN,
    /// British Indian Ocean Territory.
    IO,
    /// Iraq.
    IQ,
    /// Iran.
    IR,
    /// Iceland.
    IS,
    /// Italy.
    IT,
    /// Jersey.
    JE,
    /// Jamaica.
    JM,
    /// Jordan.
    JO,
    /// Japan.
    JP,
    /// Kenya.
    KE,
    /// Kyrgyzstan.
    KG,
    /// Cambodia.
    KH,
    /// Kiribati.
    KI,
    /// Comoros.
    KM,
    /// St. Kitts & Nevis.
    KN,
    /// North Korea.
    KP,
    /// South Korea.
    KR,
    /// Kuwait.
    KW,
    /// Cayman Islands.
    KY,
    /// Kazakhstan.
    KZ,
    /// Laos.
    LA,
    /// Lebanon.
    LB,
    /// St. Lucia.
    LC,
    /// Liechtenstein.
    LI,
    /// Sri Lanka.
    LK,
    /// Liberia.
    LR,
    /// Lesotho.
    LS,
    /// Lithuania.
    LT,
    /// Luxembourg.
    LU,
    /// Latvia.
    LV,
    /// Libya.
    LY,
    /// Morocco.
    MA,
    /// Monaco.
    MC,
    /// Moldova.
    MD,
    /// Montenegro.
    ME,
    /// St. Martin.
    MF,
    /// Madagascar.
    MG,
    /// North Macedonia.
    MK,
    /// Mali.
    ML,
    /// Myanmar (Burma).
    MM,
    /// Mongolia.
    MN,
    /// Macao SAR.
    MO,
    /// Martinique.
    MQ,
    /// Mauritania.
    MR,
    /// Montserrat.
    MS,
    /// Malta.
    MT,
    /// Mauritius.
    MU,
    /// Maldives.
    MV,
    /// Malawi.
    MW,
    /// Mexico.
    MX,
    /// Malaysia.
    MY,
    /// Mozambique.
    MZ,
    /// Namibia.
    NA,
    /// New Caledonia.
    NC,
    /// Niger.
    NE,
    /// Norfolk Island.
    NF,
    /// Nigeria.
    NG,
    /// Nicaragua.
    NI,
    /// Netherlands.
    NL,
    /// Norway.
    NO,
    /// Nepal.
    NP,
    /// Nauru.
    NR,
    /// Niue.
    NU,
    /// New Zealand.
    NZ,
    /// Oman.
    OM,
    /// Panama.
    PA,
    /// Peru.
    PE,
    /// French Polynesia.
    PF,
    /// Papua New Guinea.
    PG,
    /// Philippines.
    PH,
    /// Pakistan.
    PK,
    /// Poland.
    PL,
    /// St. Pierre & Miquelon.
    PM,
    /// Pitcairn Islands.
    PN,
    /// Palestinian Territories.
    PS,
    /// Portugal.
    PT,
    /// Paraguay.
    PY,
    /// Qatar.
    QA,
    /// Réunion.
    RE,
    /// Romania.
    RO,
    /// Serbia.
    RS,
    /// Russia.
    RU,
    /// Rwanda.
    RW,
    /// Saudi Arabia.
    SA,
    /// Solomon Islands.
    SB,
    /// Seychelles.
    SC,
    /// Sudan.
    SD,
    /// Sweden.
    SE,
    /// Singapore.
    SG,
    /// St. Helena.
    SH,
    /// Slovenia.
    SI,
    /// Svalbard & Jan Mayen.
    SJ,
    /// Slovakia.
    SK,
    /// Sierra Leone.
    SL,
    /// San Marino.
    SM,
    /// Senegal.
    SN,
    /// Somalia.
    SO,
    /// Suriname.
    SR,
    /// South Sudan.
    SS,
    /// São Tomé & Príncipe.
    ST,
    /// El Salvador.
    SV,
    /// Sint Maarten.
    SX,
    /// Syria.
    SY,
    /// Eswatini.
    SZ,
    /// Tristan da Cunha.
    TA,
    /// Turks & Caicos Islands.
    TC,
    /// Chad.
    TD,
    /// French Southern Territories.
    TF,
    /// Togo.
    TG,
    /// Thailand.
    TH,
    /// Tajikistan.
    TJ,
    /// Tokelau.
    TK,
    /// Timor-Leste.
    TL,
    /// Turkmenistan.
    TM,
    /// Tunisia.
    TN,
    /// Tonga.
    TO,
    /// Turkey.
    TR,
    /// Trinidad & Tobago.
    TT,
    /// Tuvalu.
    TV,
    /// Taiwan.
    TW,
    /// Tanzania.
    TZ,
    /// Ukraine.
    UA,
    /// Uganda.
    UG,
    /// U.S. Outlying Islands.
    UM,
    /// United States.
    US,
    /// Uruguay.
    UY,
    /// Uzbekistan.
    UZ,
    /// Vatican City.
    VA,
    /// St. Vincent & Grenadines.
    VC,
    /// Venezuela.
    VE,
    /// British Virgin Islands.
    VG,
    /// Vietnam.
    VN,
    /// Vanuatu.
    VU,
    /// Wallis & Futuna.
    WF,
    /// Samoa.
    WS,
    /// Kosovo.
    XK,
    /// Yemen.
    YE,
    /// Mayotte.
    YT,
    /// South Africa.
    ZA,
    /// Zambia.
    ZM,
    /// Zimbabwe.
    ZW,
    /// Unknown Region.
    ZZ,
}