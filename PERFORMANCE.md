| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 2.6 ± 0.1 | 2.5 | 3.8 | 1.17 ± 0.05 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.2 ± 0.1 | 2.1 | 2.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 2.5 ± 0.1 | 2.4 | 3.7 | 1.15 ± 0.07 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.2 ± 0.1 | 2.1 | 3.9 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 2.5 ± 0.1 | 2.4 | 5.3 | 1.15 ± 0.07 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.2 ± 0.1 | 2.0 | 3.9 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 91.8 ± 5.5 | 87.0 | 164.2 | 5.07 ± 0.33 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 18.1 ± 0.5 | 16.9 | 24.1 | 1.00 |

