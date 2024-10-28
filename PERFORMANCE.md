| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 20.4 ± 0.4 | 19.6 | 27.5 | 7.41 ± 0.48 |
| `echo '[1, [2], [[3]]]' \| jql '..'` | 2.7 ± 0.2 | 2.2 | 4.1 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 20.3 ± 0.5 | 19.5 | 28.0 | 7.76 ± 0.51 |
| `echo '[1, 2, 3]' \| jql '[0]'` | 2.6 ± 0.2 | 2.1 | 3.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 20.1 ± 0.5 | 19.2 | 27.6 | 7.66 ± 0.55 |
| `echo '{ "foo": "bar" }' \| jql '"foo"'` | 2.6 ± 0.2 | 2.2 | 4.1 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 122.4 ± 5.0 | 115.9 | 159.1 | 6.54 ± 0.33 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '\|>{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 18.7 ± 0.6 | 17.3 | 24.2 | 1.00 |

