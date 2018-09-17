import classNames from "classnames";
import { Component, FormEvent } from "inferno";
import { isNumber } from "../utils/util";

interface Column {
  value?: string | number;
  jsx?: JSX.Element | undefined;
  width?: number;
}

interface Row {
  key: string;
  columns: Column[];
}

interface ExtendedRow<T> extends Row {
  data: T;
}

interface Props<T> {
  headers: string[];
  data: T[];
  toRow: (data: T) => Row;
  initialSort?: string;
  initialAscending?: boolean;
  onRowClick?: (data: T) => void;
}

interface State {
  sort: string;
  ascending: boolean;
  filter: string;
}

function rowSorter(index: number, ascending: boolean): (a: Row, b: Row) => number {
  const sorter = (a: Row, b: Row) => {
    const av = a.columns[index].value;
    const bv = b.columns[index].value;
    if (av === undefined && bv === undefined) {
      return -1;
    }
    if (av === undefined) {
      return -1;
    }
    if (bv === undefined) {
      return 1;
    }
    if (isNumber(av) && !isNumber(bv)) {
      return -1;
    } else if (!isNumber(av) && isNumber(bv)) {
      return 1;
    } else if (isNumber(av) && isNumber(bv)) {
      return av - bv;
    } else {
      if (av < bv) {
        return -1;
      } else if (av > bv) {
        return 1;
      } else {
        return 0;
      }
    }
  };
  if (ascending) {
    return sorter;
  } else {
    return (a: Row, b: Row) => -1 * sorter(a, b);
  }
}

export class Table<T> extends Component<Props<T>, State> {
  public state: State = {
    sort:
      this.props.initialSort !== undefined
        ? this.props.initialSort
        : this.props.headers.find(h => h.length > 0),
    ascending: this.props.initialAscending !== undefined ? this.props.initialAscending : true,
    filter: "",
  };

  constructor(props: null, context: null) {
    super(props, context);

    this.setFilter = this.setFilter.bind(this);
  }

  public render() {
    // Map data to row
    let rows = this.props.data.map(d => {
      const row = this.props.toRow(d) as ExtendedRow<T>;
      row.data = d;
      return row;
    });
    // Filter the rows
    rows = rows.filter(row => {
      for (const column of row.columns) {
        const value = column.value;
        if (value !== undefined && value.toString().includes(this.state.filter)) {
          return true;
        }
      }
      return false;
    });
    // Sort the rows
    if (this.state.sort !== undefined) {
      const index = this.props.headers.findIndex(h => h === this.state.sort);
      rows = rows.sort(rowSorter(index, this.state.ascending));
    }
    return (
      <div>
        <div class="level is-marginless">
          <div class="level-left">{this.props.children}</div>

          <div class="level-right">
            <input
              class="input is-small"
              type="text"
              placeholder="Filter..."
              value={this.state.filter}
              onInput={this.setFilter}
            />
            <span style={{ marginLeft: "5px" }}>{`${rows.length}/${this.props.data.length}`}</span>
          </div>
        </div>

        <table class="table is-striped is-hoverable is-narrow is-fullwidth">
          <thead>
            <tr>
              {this.props.headers.map(h => {
                const hasHeader = h.length > 0;
                if (h === this.state.sort) {
                  return (
                    <th
                      className={classNames({ clickable: hasHeader })}
                      onClick={() => {
                        this.setState({ ascending: !this.state.ascending });
                      }}
                    >
                      {h}
                      {this.state.ascending ? (
                        <span class="arrow-down" />
                      ) : (
                        <span class="arrow-up" />
                      )}
                    </th>
                  );
                } else {
                  return (
                    <th
                      className={classNames({ clickable: hasHeader })}
                      onClick={() => {
                        if (hasHeader) {
                          this.setState({ sort: h });
                        }
                      }}
                    >
                      {h}
                    </th>
                  );
                }
              })}
            </tr>
          </thead>
          <tbody $HasKeyedChildren>
            {rows.length > 0
              ? rows.map((r, i) => {
                  const clickable = this.props.onRowClick !== undefined;
                  return (
                    <tr
                      className={classNames({ clickable })}
                      onClick={() => {
                        if (clickable) {
                          this.props.onRowClick(r.data);
                        }
                      }}
                      key={r.key}
                    >
                      {r.columns.map(c => {
                        let contents: string | number | JSX.Element = "";
                        if (c.jsx !== undefined) {
                          contents = c.jsx;
                        } else if (c.value !== undefined) {
                          contents = c.value;
                        }
                        return <td style={{ width: c.width }}>{contents}</td>;
                      })}
                    </tr>
                  );
                })
              : [
                  <tr key="no-elements">
                    <td
                      colSpan={this.props.headers.length}
                      class="has-text-centered has-text-weight-bold"
                    >
                      No items found
                    </td>
                  </tr>,
                ]}
          </tbody>
        </table>
      </div>
    );
  }

  private setFilter(event: FormEvent<HTMLInputElement>) {
    this.setState({ filter: event.currentTarget.value });
  }
}
