import redis
from textual.app import App, ComposeResult
from textual.containers import Horizontal
from textual.widgets import Tree, ProgressBar, Label, Input, Button
from textual.widgets.tree import TreeNode
from textual.reactive import reactive

r = redis.Redis(decode_responses=True)

class ReactiveLabel(Label):
    text_value = reactive("Initial text")

    def watch_text_value(self, new_value: str) -> None:
        self.update(new_value)

class QueryVisualizerApp(App):
    CSS = "Input { width: 70%; }" \
    "Label { margin: 0 2 0 0; }" \
    "Horizontal { height: auto; }"
    current_index = reactive(0)

    def __init__(self, record):
        super().__init__()
        self.record = record

    def compose(self) -> ComposeResult:
        self.tree_map: dict[str, (TreeNode, str)] = {}
        tree = Tree(self.record[1][0][2])
        self.tree_map[self.record[1][0][0]] = (tree.root, self.record[1][0][2])
        for row in self.record[1][1:]:
            self.tree_map[row[0]] = (self.tree_map[row[1]][0].add_leaf(row[2]), row[2])
        row = self.record[0][0]
        self.tree_map[row[0]][0].label += f" ({row[2]})"
        tree.root.expand_all()
        yield tree
        label = ReactiveLabel()
        label.text_value = f"Step: {self.current_index}"
        yield Horizontal(label, ProgressBar(len(self.record[0]), show_eta=0, show_percentage=False))
        yield Horizontal(Input(placeholder="Enter query"), Button("Execute", id="execute_button"))

    def on_button_pressed(self, event: Button.Pressed) -> None:
        try:
            record = r.execute_command("GRAPH.RECORD", "g", self.query_one(Input).value)
            self.record = record
            self.current_index = 0
        except:
            pass
        
        self.refresh(recompose=True)

    def update_tree(self):
        self.query_one(ReactiveLabel).text_value = f"Step: {self.current_index}"
        for node in self.tree_map.values():
            node[0].label = node[1]
        row = self.record[0][self.current_index]
        self.tree_map[row[0]][0].label += f" ({row[2]})"
    
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

if __name__ == "__main__":
    res = r.execute_command("GRAPH.RECORD", "g", "UNWIND range(1, 10) AS x UNWIND range(x, 10) AS y RETURN x, y")
    app = QueryVisualizerApp(res)
    app.run()