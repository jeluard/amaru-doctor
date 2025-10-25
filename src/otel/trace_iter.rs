use crate::otel::{SpanId, graph::TraceGraph};
use std::collections::VecDeque;

/// Provides a depth-first, time-ordered traversal of a span tree.
pub struct TraceIter<'a> {
    graph: &'a TraceGraph,
    // A stack of nodes to visit next. We use VecDeque to efficiently pop from
    // the front.
    to_visit: VecDeque<SpanId>,
}

impl<'a> TraceIter<'a> {
    /// This resulting iter will include the Spans in to_visit (unlike
    /// AncestorIter)
    pub fn new(graph: &'a TraceGraph, to_visit: VecDeque<SpanId>) -> Self {
        Self { graph, to_visit }
    }
}

impl<'a> Iterator for TraceIter<'a> {
    type Item = SpanId;

    fn next(&mut self) -> Option<Self::Item> {
        // Pop the next span ID from the front of the queue, we'll return this.
        let span_id = self.to_visit.pop_front()?;

        // Find the corresponding node to get its children.
        if let Some(subtree) = self.graph.subtrees.get(&span_id) {
            // We get the children in start-time order (oldest to youngest). To preserve
            // this, we reverse and push onto the stack youngest to oldest.
            for children_vec in subtree.children().values().rev() {
                // Recall: this is a Vec in the unlikely case there are two
                // children with the same start
                for child_id in children_vec.iter() {
                    // Put this node's children at the front for DFS
                    self.to_visit.push_front(*child_id);
                }
            }
        }

        Some(span_id)
    }
}
