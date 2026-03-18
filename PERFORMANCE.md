| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 2.7 ± 0.1 | 2.6 | 3.2 | 1.06 ± 0.05 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.6 ± 0.1 | 2.3 | 2.9 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 2.6 ± 0.1 | 2.5 | 3.6 | 1.03 ± 0.05 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.6 ± 0.1 | 2.4 | 3.0 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 2.6 ± 0.1 | 2.5 | 4.1 | 1.05 ± 0.05 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.5 ± 0.1 | 2.3 | 2.9 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 93.9 ± 6.1 | 87.3 | 163.2 | 6.43 ± 0.44 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 14.6 ± 0.3 | 14.0 | 19.3 | 1.00 |

