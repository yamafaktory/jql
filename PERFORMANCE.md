| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 20.0 ± 0.4 | 19.4 | 22.3 | 7.44 ± 0.78 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.7 ± 0.3 | 2.3 | 9.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 20.1 ± 0.3 | 19.4 | 22.3 | 7.40 ± 0.52 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.7 ± 0.2 | 2.4 | 4.1 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 19.9 ± 0.4 | 19.2 | 24.2 | 7.28 ± 0.56 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.7 ± 0.2 | 2.4 | 4.2 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 121.4 ± 4.9 | 115.6 | 176.7 | 6.57 ± 0.34 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 18.5 ± 0.6 | 16.9 | 23.9 | 1.00 |

