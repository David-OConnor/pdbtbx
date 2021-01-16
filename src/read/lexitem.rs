/// A definition of all lines that a PDB file can contain (and can be parsed by this program)
/// with all properties saved as primitive data types.
///
/// See wwPDB v3.30 for detailed explanation of the meaning of all fields
#[derive(Debug)]
pub enum LexItem {
    /// A REMARK saved as the remark-type-number and the remark line itself
    Remark(usize, String),
    /// An Atom with all its information, including the deprecated and rarely used fields.
    /// * hetatom (true) or atom (false)
    /// * serial number
    /// * name
    /// * alternate location
    /// * residue name
    /// * chain id
    /// * residue serial number
    /// * insertion
    /// * x
    /// * y
    /// * z
    /// * occupancy
    /// * b_factor
    /// * segment id
    /// * element
    /// * charge
    Atom(
        bool,
        usize,
        [char; 4],
        char,
        [char; 3],
        char,
        usize,
        char,
        f64,
        f64,
        f64,
        f64,
        f64,
        [char; 4],
        [char; 2],
        isize,
    ),
    /// An Anisou record with all its information, including the deprecated and rarely used fields.
    /// * hetatom (true) or atom (false)
    /// * serial number
    /// * name
    /// * alternate location
    /// * residue name
    /// * chain id
    /// * residue serial number
    /// * insertion
    /// * temperature factors
    /// * segment id
    /// * element
    /// * charge
    Anisou(
        usize,
        [char; 4],
        char,
        [char; 3],
        char,
        usize,
        char,
        [[f64; 3]; 2],
        [char; 4],
        [char; 2],
        isize,
    ),
    /// A SCALEn line, as the row (1/2/3) and data
    Scale(usize, [f64; 4]),
    /// A ORIGXn line, as the row (1/2/3) and data
    OrigX(usize, [f64; 4]),
    /// A MTRIXn line, as the row (1/2/3), serial number, data, and contained fields
    MtriX(usize, usize, [f64; 4], bool),
    /// A CRYST1 line, containing: a, b, c, alpha, beta, gamma, space group character, and space group symbols as numbers
    Crystal(f64, f64, f64, f64, f64, f64, String, usize),
    /// A MODEL with its serial number
    Model(usize),
    /// The Master record, having a checksum of the number of selected record types, used for verification
    Master(
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
    ),
    /// A SEQRES row
    Seqres(usize, char, usize, Vec<String>),
    /// A DBREF row in the original/standard format
    Dbref(
        [char; 4],
        char,
        (usize, char, usize, char),
        String,
        String,
        String,
        (usize, char, usize, char),
    ),
    /// A SEQADV row
    Seqadv(
        [char; 4],
        char,
        [char; 3],
        usize,
        char,
        String,
        String,
        Option<([char; 3], usize)>,
        String,
    ),
    /// ENDMODEL, end of the current model
    EndModel(),
    /// TER =, termination of ATOM lines to allow for HETATMs to be defined
    TER(),
    /// END, end of the whole file
    End(),
    /// Empty line, just ignore
    Empty(),
}
