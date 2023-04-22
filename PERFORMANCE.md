| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 26.7 ± 0.8 | 25.4 | 33.7 | 14.22 ± 1.70 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 1.9 ± 0.2 | 1.6 | 3.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 26.5 ± 0.8 | 25.2 | 33.9 | 14.37 ± 2.42 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 1.8 ± 0.3 | 1.6 | 6.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 26.6 ± 0.8 | 25.2 | 32.8 | 14.08 ± 1.94 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 1.9 ± 0.3 | 1.6 | 5.8 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 153.3 ± 2.2 | 150.7 | 169.0 | 7.16 ± 0.86 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 21.4 ± 2.6 | 17.5 | 47.9 | 1.00 |

