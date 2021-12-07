
// Number of vertices
const V: usize = 9;

/**
 * Steps: 
 * 
 * 1. create two arrays: one to store distances from the source,
 *                       and another to store whether vertex was visited
 * 
 * 2. calculate minimum distance of vertex from source
 * 
 * 3. update distance if vertex is not visited, vertex is not
 *    u8::MAX and the distance to that vertex on that specific arm is smaller
 *    than the recorded distance.
 * 
 * 4. Prints graph
 */
fn djikstra(graph: &[[u8; V]; V], src: usize) {
    let mut dist: [u8; V] = [u8::MAX; V];
    let mut visited: [bool; V] = [false; V];

    //  modify source distance to be 0
    dist[src] = 0;

    //  calculate min distance for each vertex
    for _ in 0..(V-1) {
        //  Get calculated minimum distance
        let visited_index = minimum_distance(&dist, &visited);
        visited[visited_index] = true;

        for v in 0..V {
            if !visited[v] &&
                graph[visited_index][v] != 0 &&
                dist[visited_index] != u8::MAX &&
                dist[visited_index] + graph[visited_index][v] < dist[v] {
                    //  update distance
                    dist[v] = dist[visited_index] + graph[visited_index][v];
                }
        }
    }

    print_graph(&dist)
}

fn minimum_distance(dist: &[u8], visited: &[bool]) -> usize {
    let mut min = u8::MAX;
    let mut min_index: isize = -1;

    for v in 0..V {
        if !visited[v] && dist[v] <= min {
            min = dist[v];
            min_index = v as isize;
        }
    }

    min_index as usize
}

fn print_graph(dist: &[u8]) {
    println!("Vertex Distance from source");

    let i: usize = 0;

    for node in dist {
        print!("{} \t\t {} \n", i, *node);
    }
}

fn main() {
    //  Create 2D array
    let graph : [[u8; V]; V] = [
        [0, 4, 0, 0, 0, 0, 0, 8, 0],
        [4, 0, 8, 0, 0, 0, 0, 11, 0],
        [0, 8, 0, 7, 0, 4, 0, 0, 2],
        [0, 0, 7, 0, 9, 14, 0, 0, 0],
        [0, 0, 0, 9, 0, 10, 0, 0, 0],
        [0, 0, 4, 14, 10, 0, 2, 0, 0],
        [0, 0, 0, 0, 0, 2, 0, 1, 6],
        [8, 11, 0, 0, 0, 0, 1, 0, 7],
        [0, 0, 2, 0, 0, 0, 6, 7, 0],
    ];

    djikstra(&graph, 0);
}
