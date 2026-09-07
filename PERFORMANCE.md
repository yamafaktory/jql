| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 2.5 ± 0.1 | 2.5 | 3.9 | 1.28 ± 0.24 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.0 ± 0.4 | 1.4 | 4.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 2.5 ± 0.1 | 2.4 | 3.3 | 1.27 ± 0.24 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.0 ± 0.4 | 1.4 | 4.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 2.5 ± 0.1 | 2.4 | 3.8 | 1.23 ± 0.22 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.0 ± 0.4 | 1.4 | 3.8 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 95.2 ± 6.3 | 88.2 | 111.4 | 6.37 ± 1.79 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 14.9 ± 4.1 | 13.0 | 82.8 | 1.00 |

