import redis
from textual.app import App, ComposeResult
from textual.containers import Horizontal
from textual.widgets import Tree, ProgressBar, Label, Input
from textual.widgets.tree import TreeNode
from textual.reactive import reactive

r = redis.Redis(decode_responses=True)

class ReactiveLabel(Label):
    text_value = reactive("Initial text")

    def watch_text_value(self, new_value: str) -> None:
        self.update(new_value)

class QueryVisualizerApp(App):
    CSS = "Label { margin: 0 2 0 0; }" \
    "Horizontal { height: auto; }"
    current_index = reactive(0)
    last_query = None

    def __init__(self, query: str):
        super().__init__()
        self.record = []
        self.query_string = []
        self.run_query(query)

    def compose(self) -> ComposeResult:
        query = ReactiveLabel(id="query_label")
        query.text_value = self.query_string[self.last_query if self.last_query is not None else -1]
        yield query
        self.tree_map: dict[str, (TreeNode, str)] = {}
        tree = Tree(self.record[1][0][2])
        tree.root.allow_expand = False
        self.tree_map[self.record[1][0][0]] = (tree.root, self.record[1][0][2])
        for row in self.record[1][1:]:
            self.tree_map[row[0]] = (self.tree_map[row[1]][0].add_leaf(row[2]), row[2])
        row = self.record[0][0]
        self.tree_map[row[0]][0].label += f" | Env: ({row[2]})"
        tree.root.expand_all()
        yield tree
        label = ReactiveLabel(id="step_label")
        label.text_value = f"Step: {self.current_index + 1}/{len(self.record[0])}"
        yield Horizontal(label, ProgressBar(len(self.record[0]), show_eta=0, show_percentage=False))
        yield Input(placeholder="Enter query")

    def on_ready(self):
        row = self.record[0][0]
        self.query_one(Tree).select_node(self.tree_map[row[0]][0])

    def run_query(self, query: str) -> None:
        try:
            record = r.execute_command("GRAPH.RECORD", "g", query)
            self.record = record
            self.current_index = 0
            self.query_string.append(query)
        except:
            pass
        
        self.refresh(recompose=True)
    
    def on_input_submitted(self, event: Input.Submitted) -> None:
        self.run_query(event.value)

    def update_tree(self):
        self.get_widget_by_id("step_label").text_value = f"Step: {self.current_index + 1}/{len(self.record[0])}"
        for node in self.tree_map.values():
            node[0].label = node[1]
        row = self.record[0][self.current_index]
        self.tree_map[row[0]][0].label += f" | Env: ({row[2]})"
        self.query_one(Tree).select_node(self.tree_map[row[0]][0])
    
    def on_key(self, event) -> None:
        if event.key == "left":
            if self.current_index > 0:
                self.current_index = self.current_index - 1
                self.query_one(ProgressBar).advance(-1)
                self.update_tree()
        elif event.key == "right":
            if self.current_index < len(self.record[0]) - 1:
                self.current_index = self.current_index + 1
                self.query_one(ProgressBar).advance(1)
                self.update_tree()
        elif event.key == "up":
            if self.last_query is None:
                self.last_query = len(self.query_string) - 1
            else:
                self.last_query = max(0, self.last_query - 1)
            self.query_one(Input).value = self.query_string[self.last_query]
        elif event.key == "down":
            if self.last_query is not None:
                if self.last_query == len(self.query_string) - 1:
                    self.last_query = None
                    self.query_one(Input).clear()
                else:
                    self.last_query = min(len(self.query_string) - 1, self.last_query + 1)
                    self.query_one(Input).value = self.query_string[self.last_query] if self.last_query is not None else ""

if __name__ == "__main__":
    app = QueryVisualizerApp("UNWIND range(1, 10) AS x UNWIND range(x, 10) AS y RETURN x, y")
    app.run()