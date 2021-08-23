// DOM nodes
struct Node {
    children: Vec<Node>,

    node_type: NodeType,
}

// Subtypes of Node
enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

type AttrMap = HashMap<String, String>;

// Constructor for a DOM node, type Text
fn text(data: String) -> Node {
    Node { children: Vec::new(), node_type: NodeType::Text(data)}
}

// Constructor for a DOM node, type Element
fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attrs,
        }),
    }
}