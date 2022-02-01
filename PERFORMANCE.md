| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 40.2 ± 1.9 | 37.4 | 51.6 | 16.86 ± 4.05 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.4 ± 0.6 | 1.9 | 12.3 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 40.1 ± 2.0 | 36.5 | 52.9 | 15.64 ± 6.03 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.6 ± 1.0 | 1.9 | 15.1 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 39.7 ± 1.6 | 37.2 | 51.9 | 19.13 ± 5.42 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.1 ± 0.6 | 1.5 | 14.7 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 200.1 ± 5.7 | 188.5 | 240.0 | 1.30 ± 0.06 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 153.6 ± 5.4 | 141.8 | 188.8 | 1.00 |

