| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, [2], [[3]]]' \| jq 'flatten'` | 39.8 ± 1.1 | 37.3 | 48.2 | 16.41 ± 2.20 |
| `echo '[1, [2], [[3]]]' \| jql '...'` | 2.4 ± 0.3 | 1.9 | 4.5 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '[1, 2, 3]' \| jq '.[0]'` | 39.5 ± 1.2 | 36.1 | 49.0 | 17.22 ± 2.16 |
| `echo '[1, 2, 3]' \| jql '.[0]'` | 2.3 ± 0.3 | 1.9 | 4.6 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `echo '{ "foo": "bar" }' \| jq '.foo'` | 39.7 ± 1.2 | 37.6 | 49.2 | 17.33 ± 2.06 |
| `echo '{ "foo": "bar" }' \| jql '."foo"'` | 2.3 ± 0.3 | 1.9 | 3.4 | 1.00 |

| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jq -r '[.[] \| {name: .name, url: .url, language: .language, stargazers_count: .stargazers_count, watchers_count: .watchers_count}]' > /dev/null` | 196.5 ± 3.3 | 180.4 | 213.4 | 1.31 ± 0.04 |
| `cat /home/runner/work/jql/jql/assets/github-repositories.json \| jql '.\|{"name", "url", "language", "stargazers_count", "watchers_count"}' > /dev/null` | 150.5 ± 3.8 | 138.8 | 200.6 | 1.00 |

