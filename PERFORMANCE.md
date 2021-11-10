| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 33.0 ± 2.3 | 27.9 | 91.4 | 16.95 ± 3.11 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 1.9 ± 0.3 | 1.5 | 4.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 32.9 ± 1.0 | 28.3 | 40.1 | 16.35 ± 3.62 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.0 ± 0.4 | 1.6 | 9.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 32.9 ± 1.3 | 28.3 | 45.5 | 15.89 ± 3.13 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.1 ± 0.4 | 1.6 | 4.8 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 168.8 ± 24.6 | 146.8 | 491.6 | 1.29 ± 0.19 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 131.4 ± 3.6 | 117.5 | 154.8 | 1.00 |

