/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/


use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>, // 邻接表表示图的连接关系
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n], // 初始化邻接表
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest); // 添加边到 src 的邻接列表
        self.adj[dest].push(src); // 无向图需要双向添加边
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        let mut visited = vec![false; self.adj.len()]; // 记录节点是否被访问过
        let mut visit_order = vec![]; // 存储访问节点的顺序
        let mut queue = VecDeque::new(); // 使用双端队列作为广度优先搜索的队列

        // 将起始节点加入队列并标记为已访问
        queue.push_back(start);
        visited[start] = true;

        // 进行广度优先搜索
        while let Some(node) = queue.pop_front() {
            visit_order.push(node); // 将当前节点加入访问顺序列表

            // 遍历当前节点的邻接节点
            for &neighbor in &self.adj[node] {
                // 如果邻接节点未被访问，则加入队列并标记为已访问
                if !visited[neighbor] {
                    queue.push_back(neighbor);
                    visited[neighbor] = true;
                }
            }
        }

        visit_order // 返回访问节点的顺序
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}

